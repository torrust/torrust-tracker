#!/usr/bin/env python3
# Copyright (c) 2016-2017 The Bitcoin Core developers
# Distributed under the MIT software license, see the accompanying
# file COPYING or http://www.opensource.org/licenses/mit-license.php.

# This script will locally construct a merge commit for a pull request on a
# github repository, inspect it, sign it and optionally push it.

# The following temporary branches are created/overwritten and deleted:
# * pull/$PULL/base (the current master we're merging onto)
# * pull/$PULL/head (the current state of the remote pull request)
# * pull/$PULL/merge (github's merge)
# * pull/$PULL/local-merge (our merge)

# In case of a clean merge that is accepted by the user, the local branch with
# name $BRANCH is overwritten with the merged result, and optionally pushed.
import os
from sys import stdin,stdout,stderr
import argparse
import re
import hashlib
import subprocess
import sys
import json
import codecs
import unicodedata
from urllib.request import Request, urlopen
from urllib.error import HTTPError

# External tools (can be overridden using environment)
GIT = os.getenv('GIT','git')
SHELL = os.getenv('SHELL','bash')

# OS specific configuration for terminal attributes
ATTR_RESET = ''
ATTR_PR = ''
ATTR_NAME = ''
ATTR_WARN = ''
ATTR_HL = ''
COMMIT_FORMAT = '%H %s (%an)%d'
if os.name == 'posix': # if posix, assume we can use basic terminal escapes
    ATTR_RESET = '\033[0m'
    ATTR_PR = '\033[1;36m'
    ATTR_NAME = '\033[0;36m'
    ATTR_WARN = '\033[1;31m'
    ATTR_HL = '\033[95m'
    COMMIT_FORMAT = '%C(bold blue)%H%Creset %s %C(cyan)(%an)%Creset%C(green)%d%Creset'

def sanitize(s, newlines=False):
    '''
    Strip control characters (optionally except for newlines) from a string.
    This prevent text data from doing potentially confusing or harmful things
    with ANSI formatting, linefeeds bells etc.
    '''
    return ''.join(ch for ch in s if unicodedata.category(ch)[0] != "C" or (ch == '\n' and newlines))

def quote_tree_value(value):
    '''
    Quote a path or a link target read out of a tree, for a message that reports it.

    Tree content is chosen by whoever wrote the commit, and a path or a link target may
    legally carry a newline or a terminal escape. Interpolated as it stands, such a value
    forges lines of this report: a target ending in a newline and the text of an error can
    print a refusal the tool never made, or hide one it did. The quoted form escapes those
    characters and delimits the value, so a reader can see where it starts and ends, and it
    is only ever a rendering: every comparison this check makes runs on the bytes themselves.

    The value is the tree's bytes, which need not be valid UTF-8, so the bytes that are not
    are rendered as escapes rather than replaced, and the rendering stays reversible.
    '''
    return repr(value.decode('utf-8', errors='backslashreplace'))

def git_config_get(option, default=None):
    '''
    Get named configuration option from git repository.
    '''
    try:
        return subprocess.check_output([GIT,'config','--get',option]).rstrip().decode('utf-8')
    except subprocess.CalledProcessError:
        return default

def get_response(req_url, ghtoken):
    req = Request(req_url)
    if ghtoken is not None:
        req.add_header('Authorization', 'token ' + ghtoken)
    return urlopen(req)

def sanitize_ghdata(rec):
    '''
    Sanitize comment/review record coming from github API in-place.
    This currently sanitizes the following:
    - ['title'] PR title (optional, may not have newlines)
    - ['body'] Comment body (required, may have newlines)
    It also checks rec['user']['login'] (required) to be a valid github username.

    When anything more is used, update this function!
    '''
    if 'title' in rec: # only for PRs
        rec['title'] = sanitize(rec['title'], newlines=False)
    if rec['body'] is None:
        rec['body'] = ''
    rec['body'] = sanitize(rec['body'], newlines=True)

    if rec['user'] is None: # User deleted account
        rec['user'] = {'login': '[deleted]'}
    else:
        # "Github username may only contain alphanumeric characters or hyphens'.
        # Sometimes bot have a "[bot]" suffix in the login, so we also match for that
        # Use \Z instead of $ to not match final newline only end of string.
        if not re.match(r'[a-zA-Z0-9-]+(\[bot\])?\Z', rec['user']['login'], re.DOTALL):
            raise ValueError('Github username contains invalid characters: {}'.format(sanitize(rec['user']['login'])))
    return rec

def retrieve_json(req_url, ghtoken, use_pagination=False):
    '''
    Retrieve json from github.
    Return None if an error happens.
    '''
    try:
        reader = codecs.getreader('utf-8')
        if not use_pagination:
            return sanitize_ghdata(json.load(reader(get_response(req_url, ghtoken))))

        obj = []
        page_num = 1
        while True:
            req_url_page = '{}?page={}'.format(req_url, page_num)
            result = get_response(req_url_page, ghtoken)
            obj.extend(json.load(reader(result)))

            link = result.headers.get('link', None)
            if link is not None:
                link_next = [l for l in link.split(',') if 'rel="next"' in l]
                if len(link_next) > 0:
                    page_num = int(link_next[0][link_next[0].find("page=")+5:link_next[0].find(">")])
                    continue
            break
        return [sanitize_ghdata(d) for d in obj]
    except HTTPError as e:
        error_message = e.read()
        print('Warning: unable to retrieve pull information from github: %s' % e)
        print('Detailed error: %s' % error_message)
        return None
    except Exception as e:
        print('Warning: unable to retrieve pull information from github: %s' % e)
        return None

def retrieve_pr_info(repo,pull,ghtoken):
    req_url = "https://api.github.com/repos/"+repo+"/pulls/"+pull
    return retrieve_json(req_url,ghtoken)

def retrieve_pr_comments(repo,pull,ghtoken):
    req_url = "https://api.github.com/repos/"+repo+"/issues/"+pull+"/comments"
    return retrieve_json(req_url,ghtoken,use_pagination=True)

def retrieve_pr_reviews(repo,pull,ghtoken):
    req_url = "https://api.github.com/repos/"+repo+"/pulls/"+pull+"/reviews"
    return retrieve_json(req_url,ghtoken,use_pagination=True)

def ask_prompt(text):
    print(text,end=" ",file=stderr)
    stderr.flush()
    reply = stdin.readline().rstrip()
    print("",file=stderr)
    return reply

def get_symlink_entries(commit, target_by_blob=None):
    '''
    List the symbolic links in a commit's tree as sorted (path, target) pairs of raw bytes.

    The target is the link's literal content, which is what a declaration has to match, and
    a tree names both paths and link targets in bytes that need not be valid UTF-8. Decoding
    either would decide the match on a rendering rather than on the content: a lossy decode
    maps distinct byte sequences onto the same replacement character, and a strict one raises
    on content a repository is free to commit. Both are read as bytes and stay bytes, so the
    comparison is byte for byte as the rules state, and content that cannot be a declared
    target simply fails to match.

    '-z' asks for the paths themselves. Without it the listing is the path as git renders it,
    which quotes control characters always and non-ASCII bytes under the default
    'core.quotePath', so a declaration would have to name git's rendering rather than the path.

    'target_by_blob' carries link contents already read, so a caller walking a range reads each
    distinct content once instead of once per commit that carries it. A link usually keeps the
    same content across a whole pull request, and identical content is one blob whatever the
    path, so this turns a cost that grows with commits times links into one that grows with the
    distinct contents the range actually holds. Keyed by object id, it cannot answer for a link
    whose content changed: a changed target is a different blob and is read.
    '''
    if target_by_blob is None:
        target_by_blob = {}
    entries = []
    listing = subprocess.check_output([GIT, 'ls-tree', '--full-tree', '-r', '-z', commit])
    for record in listing.split(b'\0'):
        if not record:
            continue
        name_sep = record.index(b'\t')
        metadata = record[:name_sep].split() # perms, type, object id
        if (int(metadata[0].decode('utf-8'), 8) & 0o170000) != 0o120000:
            continue
        path = record[name_sep+1:]
        blob = metadata[2]
        if blob not in target_by_blob:
            target_by_blob[blob] = subprocess.check_output([GIT, 'cat-file', 'blob', blob.decode('utf-8')])
        entries.append((path, target_by_blob[blob]))
    return sorted(entries)

def symlink_declaration_path_error(path):
    '''
    Explain why a value cannot name a declaration inside a tree, or return None when it can.

    The argument names a location in the merged tree rather than in a filesystem, so an
    absolute value and one that escapes the repository are both rejected before any work.
    '''
    if not path:
        return "--symlinks needs a repository-relative path inside the merged tree, not an empty value"
    if path.startswith('/') or os.path.isabs(path):
        return f"--symlinks path '{path}' is absolute; it names a location inside the merged tree, which is always repository-relative"
    if '..' in path.replace('\\', '/').split('/'):
        return f"--symlinks path '{path}' escapes the repository; it names a location inside the merged tree"
    return None

def symlink_target_is_confined(target):
    '''
    Report whether a link target, given as the tree's bytes, stays inside the repository.

    An absolute target and a target containing a '..' segment are never accepted, whatever
    a declaration says, because this is a property of the target rather than of the file
    that declares it.
    '''
    if not target:
        return False
    if target.startswith(b'/'):
        return False
    return b'..' not in target.split(b'/')

# The declaration format this tool reads, as the format's own documentation states it. The
# namespace names the format rather than the repository, so every repository adopting the
# workflow carries the same value, and the version is the version of the format.
SYMLINK_DECLARATION_NAMESPACE = 'com.torrust.repository.symlinks'
SYMLINK_DECLARATION_VERSION = [1, 0, 0]

def quote_declaration_value(value):
    '''
    Render a value read out of a declaration, for a message that reports it.

    A declaration is tree content, chosen by whoever wrote the commit, so a value this report
    names could otherwise carry a newline or a terminal escape and forge a line of the report.
    The value is rendered as the JSON it came from, which escapes every control character and
    everything outside ASCII, so the rendering is printable text that cannot forge a line, and
    a reader sees the value with its type: a string keeps its quotes, and a number does not.
    '''
    return json.dumps(value)

def is_declared_literal(found, expected):
    '''
    Report whether a value read from a declaration is exactly a literal this tool expects.

    Equality alone would not answer this. Python compares True to 1 and 1 to 1.0 as equal,
    while JSON's true, 1 and 1.0 are three different documents, so a plain comparison would
    admit a header that does not carry the value the report says was checked. The type is
    required alongside the value, and a list matches element by element under the same rule.
    Python's bool is a subclass of int, which is why an integer literal excludes it explicitly.
    '''
    if isinstance(expected, str):
        return isinstance(found, str) and found == expected
    if isinstance(expected, int):
        return isinstance(found, int) and not isinstance(found, bool) and found == expected
    if isinstance(expected, list):
        return (isinstance(found, list) and len(found) == len(expected)
                and all(is_declared_literal(f, e) for f, e in zip(found, expected)))
    return False

def reject_declaration_constant(name):
    '''
    Refuse one of the constants Python's JSON reader accepts outside the JSON grammar.

    JSON has no NaN, Infinity or -Infinity, and Python's reader admits all three by default.
    A declaration is repository content, chosen by whoever wrote the commit, so without this a
    document that every conforming reader rejects would be read here and could exempt a link,
    while the rule that a declaration which is not valid JSON exempts nothing would never have
    applied to it. Raising ValueError is how that rule is reached: it is what reading a
    declaration already treats as a document it cannot read, so such a document takes the one
    path any other invalid JSON takes, with the constant that was found named in the report.
    '''
    raise ValueError(f'it carries the non-standard constant {name}')

def symlink_declaration_header_error(document):
    '''
    Explain why a document does not declare the format this tool reads, or return None when it does.

    A declaration is only read once it says what it is. The header states the format in
    'namespace' and the revision of that format in 'version', and both are checked before any
    entry is read, so a document belonging to another format, or to a revision whose rules this
    tool has not been taught, exempts nothing rather than being read under rules it was not
    written to. The check is exact in both directions: an absent field is not a supported value,
    and a value that merely resembles the supported one is not it either. A later revision of the
    format therefore has to teach this tool its version rather than pass unread, which is the
    direction a check that decides what a merge admits has to fail in.
    '''
    for field, expected in (('namespace', SYMLINK_DECLARATION_NAMESPACE),
                            ('version', SYMLINK_DECLARATION_VERSION)):
        if field not in document:
            return (f"it carries no '{field}'; this tool reads a declaration whose "
                    f"'{field}' is {quote_declaration_value(expected)}")
        found = document[field]
        if not is_declared_literal(found, expected):
            return (f"its '{field}' is {quote_declaration_value(found)} rather than "
                    f"{quote_declaration_value(expected)}")
    return None

def read_symlink_declaration(commit, path):
    '''
    Read the symbolic-link declaration at a tree path of a commit.

    Returns the declared links as a dict of path to (target, reason), together with a
    message explaining why nothing could be declared. An absent file declares nothing and
    is not a problem; a file that cannot be read as a declaration declares nothing either,
    and its message is reported so a broken file is visible instead of silently permissive.

    A document whose header does not declare this format and a supported version of it is a
    file that cannot be read as a declaration, whatever else it contains, so it takes that
    same path: nothing is declared, and the report names the field that disagreed.

    The document is read as JSON and nothing wider. Python's reader admits three constants
    JSON does not define, so it is told to refuse them; the refusal is a ValueError, which is
    what a document that is not valid JSON already raises here, so wherever in the document a
    constant appears the file is one that cannot be read and exempts nothing.
    '''
    try:
        raw = subprocess.check_output([GIT, 'show', commit+':'+path], stderr=subprocess.DEVNULL)
    except subprocess.CalledProcessError:
        return ({}, None)
    try:
        document = json.loads(raw.decode('utf-8'), parse_constant=reject_declaration_constant)
    except (UnicodeDecodeError, ValueError) as exc:
        return ({}, f'it is not valid JSON ({exc})')
    if not isinstance(document, dict):
        return ({}, 'its top level is not a JSON object')
    header_problem = symlink_declaration_header_error(document)
    if header_problem is not None:
        return ({}, header_problem)
    listed = document.get('symlinks')
    if not isinstance(listed, list):
        return ({}, "it carries no 'symlinks' array")
    declared = {}
    for entry in listed:
        if not isinstance(entry, dict):
            return ({}, 'one of its entries is not a JSON object')
        fields = [entry.get(field) for field in ('path', 'target', 'reason')]
        if not all(isinstance(value, str) and value for value in fields):
            return ({}, "an entry lacks a non-empty 'path', 'target' or 'reason'")
        declared_path, target, reason = fields
        if declared_path in declared:
            return ({}, f"it declares '{declared_path}' more than once")
        declared[declared_path] = (target, reason)
    return (declared, None)

def check_symlinks(introduced_commits, merge_commit, declaration_path):
    '''
    Refuse every symbolic link in the checked commits that the declaration does not admit.

    The checked commits are the ones the merge introduces plus the merge commit itself, and
    the declaration is read from the merge commit alone, so one reviewed statement answers
    for the whole range. Matching runs from the trees to the declaration: a declared entry
    can only remove a refusal for a link that exists, never introduce one. An entry that
    declares no link of the merged result is reported as stale, because the merged result is
    what a later change has to drop it from. Returns whether the merge may continue.
    '''
    declared = {}
    if declaration_path is not None:
        declared, problem = read_symlink_declaration(merge_commit, declaration_path)
        if problem is not None:
            print(f"WARNING: Ignoring the symlink declaration '{declaration_path}' because {problem}. No symbolic link is exempted.")
        # A declaration is JSON, so its paths and targets arrive as text, while a tree names
        # both in bytes. They are encoded once here, so every comparison below is between the
        # bytes the declaration stands for and the bytes the tree carries.
        declared = {path.encode('utf-8'): (target.encode('utf-8'), reason)
                    for path, (target, reason) in declared.items()}

    accepted = {}
    merged_paths = set()
    refusals = []
    # Shared across the walk so each distinct link content is read from the object store once
    # rather than once per commit that carries it.
    target_by_blob = {}
    for commit in list(introduced_commits) + [merge_commit]:
        entries = get_symlink_entries(commit, target_by_blob)
        if commit == merge_commit:
            merged_paths = {path for path, _ in entries}
        for path, target in entries:
            declaration = declared.get(path)
            if declaration is not None and declaration[0] == target and symlink_target_is_confined(target):
                accepted[path] = declaration
                continue
            refusals.append((path, commit))

    for path in sorted(accepted):
        target, reason = accepted[path]
        print(f"Accepted symlink: {quote_tree_value(path)} -> {quote_tree_value(target)}: {sanitize(reason)}")
    for path in sorted(set(declared) - merged_paths):
        print(f"WARNING: Declared symlink {quote_tree_value(path)} is not a symbolic link in the merged result; the entry in '{declaration_path}' is stale and can be dropped once no commit a merge introduces still carries the link.")
    for path, commit in refusals:
        print(f"ERROR: File {quote_tree_value(path)} was a symlink in commit {commit}")
    # The prompts this report has to precede are written to an unbuffered stderr, so a buffered
    # stdout would deliver the report after the maintainer has already been asked to sign.
    stdout.flush()

    return not refusals

def tree_sha512sum(commit='HEAD'):
    # request metadata for entire tree, recursively
    files = []
    blob_by_name = {}
    for line in subprocess.check_output([GIT, 'ls-tree', '--full-tree', '-r', commit]).splitlines():
        name_sep = line.index(b'\t')
        metadata = line[:name_sep].split() # perms, 'blob', blobid
        assert(metadata[1] == b'blob')
        name = line[name_sep+1:]
        files.append(name)
        blob_by_name[name] = metadata[2]

    files.sort()
    # open connection to git-cat-file in batch mode to request data for all blobs
    # this is much faster than launching it per file
    p = subprocess.Popen([GIT, 'cat-file', '--batch'], stdout=subprocess.PIPE, stdin=subprocess.PIPE)
    overall = hashlib.sha512()
    for f in files:
        blob = blob_by_name[f]
        # request blob
        p.stdin.write(blob + b'\n')
        p.stdin.flush()
        # read header: blob, "blob", size
        reply = p.stdout.readline().split()
        assert(reply[0] == blob and reply[1] == b'blob')
        size = int(reply[2])
        # hash the blob data
        intern = hashlib.sha512()
        ptr = 0
        while ptr < size:
            bs = min(65536, size - ptr)
            piece = p.stdout.read(bs)
            if len(piece) == bs:
                intern.update(piece)
            else:
                raise IOError('Premature EOF reading git cat-file output')
            ptr += bs
        dig = intern.hexdigest()
        assert(p.stdout.read(1) == b'\n') # ignore LF that follows blob data
        # update overall hash with file hash
        overall.update(dig.encode("utf-8"))
        overall.update("  ".encode("utf-8"))
        overall.update(f)
        overall.update("\n".encode("utf-8"))
    p.stdin.close()
    if p.wait():
        raise IOError('Non-zero return value executing git cat-file')
    return overall.hexdigest()

def get_acks_from_comments(head_commit, comments) -> dict:
    # Look for abbreviated commit id, because not everyone wants to type/paste
    # the whole thing and the chance of collisions within a PR is small enough
    head_abbrev = head_commit[0:6]
    acks = {}
    for c in comments:
        review = [
            l for l in c["body"].splitlines()
            if "ACK" in l
            and head_abbrev in l
            and not l.startswith("> ")  # omit if quoted comment
            and not l.startswith("    ")  # omit if markdown indentation
        ]
        if review:
            acks[c['user']['login']] = review[0]
    return acks

def make_acks_message(head_commit, acks) -> str:
    if acks:
        ack_str ='\n\nACKs for top commit:\n'.format(head_commit)
        for name, msg in acks.items():
            ack_str += '  {}:\n'.format(name)
            ack_str += '    {}\n'.format(msg)
    else:
        ack_str ='\n\nTop commit has no ACKs.\n'
    return ack_str

def print_merge_details(pull_reference, title, branch, base_branch, head_branch, acks, message):
    print('{}{}{} {} {}into {}{}'.format(ATTR_RESET+ATTR_PR,pull_reference,ATTR_RESET,title,ATTR_RESET+ATTR_PR,branch,ATTR_RESET))
    subprocess.check_call([GIT,'--no-pager','log','--graph','--topo-order','--pretty=tformat:'+COMMIT_FORMAT,base_branch+'..'+head_branch])
    if acks is not None:
        if acks:
            print('{}ACKs:{}'.format(ATTR_PR, ATTR_RESET))
            for ack_name, ack_msg in acks.items():
                print('* {} {}({}){}'.format(ack_msg, ATTR_NAME, ack_name, ATTR_RESET))
        else:
            print('{}Top commit has no ACKs!{}'.format(ATTR_WARN, ATTR_RESET))
    show_message = False
    if message is not None and '@' in message:
        print('{}Merge message contains an @!{}'.format(ATTR_WARN, ATTR_RESET))
        show_message = True
    if message is not None and '<!-' in message:
        print('{}Merge message contains an html comment!{}'.format(ATTR_WARN, ATTR_RESET))
        show_message = True
    if show_message:
        # highlight what might have tripped a warning
        message = message.replace('@', ATTR_HL + '@' + ATTR_RESET)
        message = message.replace('<!-', ATTR_HL + '<!-' + ATTR_RESET)
        print('-' * 75)
        print(message)
        print('-' * 75)

def parse_arguments():
    epilog = '''
        In addition, you can set the following git configuration variables:
        githubmerge.repository (mandatory, e.g. <owner>/<repo>),
        githubmerge.pushmirrors (default: none, comma-separated list of mirrors to push merges of the master development branch to, e.g. `git@gitlab.com:<owner>/<repo>.git,git@github.com:<owner>/<repo>.git`),
        user.signingkey (mandatory),
        user.ghtoken (default: none).
        githubmerge.merge-author-email (default: Email from git config),
        githubmerge.host (default: git@github.com),
        githubmerge.branch (no default),
        githubmerge.testcmd (default: none).
    '''
    parser = argparse.ArgumentParser(description='Utility to merge, sign and push github pull requests',
            epilog=epilog)
    parser.add_argument('--repo-from', '-r', metavar='repo_from', type=str, nargs='?',
        help='The repo to fetch the pull request from. Useful for monotree repositories. Can only be specified when branch==master. (default: githubmerge.repository setting)')
    parser.add_argument('--symlinks', metavar='tree_path', type=str, default=None,
        help='Repository-relative path, inside the merged tree, of the file declaring which symbolic links this repository accepts. Every declared link whose target matches is exempt from the symlink refusal; everything else still refuses. Without this argument no declaration is read and every symbolic link refuses. (no default)')
    parser.add_argument('pull', metavar='PULL', type=int, nargs=1,
        help='Pull request ID to merge')
    parser.add_argument('branch', metavar='BRANCH', type=str, nargs='?',
        default=None, help='Branch to merge against (default: githubmerge.branch setting, or base branch for pull, or \'master\')')
    args = parser.parse_args()
    if args.symlinks is not None:
        path_error = symlink_declaration_path_error(args.symlinks)
        if path_error is not None:
            parser.error(path_error)
    return args

def main():
    # Extract settings from git repo
    repo = git_config_get('githubmerge.repository')
    host = git_config_get('githubmerge.host','git@github.com')
    opt_branch = git_config_get('githubmerge.branch',None)
    merge_author_email = git_config_get('githubmerge.merge-author-email',None)
    testcmd = git_config_get('githubmerge.testcmd')
    ghtoken = git_config_get('user.ghtoken')
    signingkey = git_config_get('user.signingkey')
    if repo is None:
        print("ERROR: No repository configured. Use this command to set:", file=stderr)
        print("git config githubmerge.repository <owner>/<repo>", file=stderr)
        sys.exit(1)
    if signingkey is None:
        print("ERROR: No GPG signing key set. Set one using:",file=stderr)
        print("git config --global user.signingkey <key>",file=stderr)
        sys.exit(1)

    # Extract settings from command line
    args = parse_arguments()
    repo_from = args.repo_from or repo
    is_other_fetch_repo = repo_from != repo
    pull = str(args.pull[0])

    if host.startswith(('https:','http:')):
        host_repo = host+"/"+repo+".git"
        host_repo_from = host+"/"+repo_from+".git"
    else:
        host_repo = host+":"+repo
        host_repo_from = host+":"+repo_from

    # Receive pull information from github
    info = retrieve_pr_info(repo_from,pull,ghtoken)
    if info is None:
        sys.exit(1)
    title = info['title'].strip()
    body = info['body'].strip()
    pull_reference = repo_from + '#' + pull
    # precedence order for destination branch argument:
    #   - command line argument
    #   - githubmerge.branch setting
    #   - base branch for pull (as retrieved from github)
    #   - 'master'
    branch = args.branch or opt_branch or info['base']['ref'] or 'master'

    if branch == 'master':
        push_mirrors = git_config_get('githubmerge.pushmirrors', default='').split(',')
        push_mirrors = [p for p in push_mirrors if p]  # Filter empty string
    else:
        push_mirrors = []
        if is_other_fetch_repo:
            print('ERROR: --repo-from is only supported for the master development branch')
            sys.exit(1)

    # Initialize source branches
    head_branch = 'pull/'+pull+'/head'
    base_branch = 'pull/'+pull+'/base'
    merge_branch = 'pull/'+pull+'/merge'
    local_merge_branch = 'pull/'+pull+'/local-merge'

    devnull = open(os.devnull, 'w', encoding="utf8")
    try:
        subprocess.check_call([GIT,'checkout','-q',branch])
    except subprocess.CalledProcessError:
        print(f"ERROR: Cannot check out branch {branch}.", file=stderr)
        sys.exit(3)
    try:
        subprocess.check_call([GIT,'fetch','-q',host_repo_from,'+refs/pull/'+pull+'/*:refs/heads/pull/'+pull+'/*',
                                                          '+refs/heads/'+branch+':refs/heads/'+base_branch])
    except subprocess.CalledProcessError:
        print(f"ERROR: Cannot find pull request {pull_reference} or branch {branch} on {host_repo_from}.", file=stderr)
        sys.exit(3)
    try:
        subprocess.check_call([GIT,'--no-pager','log','-q','-1','refs/heads/'+head_branch], stdout=devnull, stderr=stdout)
        head_commit = subprocess.check_output([GIT,'--no-pager','log','-1','--pretty=format:%H',head_branch]).decode('utf-8')
        assert len(head_commit) == 40
    except subprocess.CalledProcessError:
        print(f"ERROR: Cannot find head of pull request {pull_reference} on {host_repo_from}.", file=stderr)
        sys.exit(3)
    try:
        subprocess.check_call([GIT,'--no-pager','log','-q','-1','refs/heads/'+merge_branch], stdout=devnull, stderr=stdout)
    except subprocess.CalledProcessError:
        print(f"ERROR: Cannot find merge of pull request {pull_reference} on {host_repo_from}.", file=stderr)
        sys.exit(3)
    subprocess.check_call([GIT,'checkout','-q',base_branch])
    subprocess.call([GIT,'branch','-q','-D',local_merge_branch], stderr=devnull)
    subprocess.check_call([GIT,'checkout','-q','-b',local_merge_branch])

    try:
        # Go up to the repository's root.
        toplevel = subprocess.check_output([GIT,'rev-parse','--show-toplevel']).strip()
        os.chdir(toplevel)
        # Create unsigned merge commit.
        if title:
            firstline = 'Merge {}: {}'.format(pull_reference,title)
        else:
            firstline = 'Merge {}'.format(pull_reference)
        message = firstline + '\n\n'
        message += subprocess.check_output([GIT,'--no-pager','log','--no-merges','--topo-order','--pretty=format:%H %s (%an)',base_branch+'..'+head_branch]).decode('utf-8')
        message += '\n\nPull request description:\n\n  ' + body.replace('\n', '\n  ') + '\n'
        try:
            subprocess.check_call([GIT,'merge','-q','--commit','--no-edit','--no-ff','--no-gpg-sign','-m',message.encode('utf-8'),head_branch])
        except subprocess.CalledProcessError:
            print("ERROR: Cannot be merged cleanly.",file=stderr)
            subprocess.check_call([GIT,'merge','--abort'])
            sys.exit(4)
        logmsg = subprocess.check_output([GIT,'--no-pager','log','--pretty=format:%s','-n','1']).decode('utf-8')
        if logmsg.rstrip() != firstline.rstrip():
            print("ERROR: Creating merge failed (already merged?).",file=stderr)
            sys.exit(4)

        # Check the symbolic links in every commit the merge introduces, not only in the
        # merged tip: a link that appears in an intermediate commit and disappears before
        # the tip still resolves on every checkout, archive, or bisect of the commit that
        # carries it. The declaration that admits them is read from the merged tree alone.
        merge_commit = subprocess.check_output([GIT,'rev-parse','HEAD']).decode('utf-8').strip()
        introduced_commits = subprocess.check_output([GIT,'--no-pager','log','--reverse','--topo-order','--pretty=format:%H',base_branch+'..'+head_branch]).decode('utf-8').split()
        if not check_symlinks(introduced_commits, merge_commit, args.symlinks):
            sys.exit(4)

        # Compute SHA512 of git tree (to be able to detect changes before sign-off)
        try:
            first_sha512 = tree_sha512sum()
        except subprocess.CalledProcessError:
            print("ERROR: Unable to compute tree hash")
            sys.exit(4)

        print_merge_details(pull_reference, title, branch, base_branch, head_branch, acks=None, message=None)
        print()

        # Run test command if configured.
        if testcmd:
            if subprocess.call(testcmd,shell=True):
                print(f"ERROR: Running '{testcmd}' failed.",file=stderr)
                sys.exit(5)

            # Show the created merge.
            diff = subprocess.check_output([GIT,'diff',merge_branch+'..'+local_merge_branch])
            subprocess.check_call([GIT,'diff',base_branch+'..'+local_merge_branch])
            if diff:
                print("WARNING: merge differs from github!",file=stderr)
                reply = ask_prompt("Type 'ignore' to continue.")
                if reply.lower() == 'ignore':
                    print("Difference with github ignored.",file=stderr)
                else:
                    sys.exit(6)
        else:
            # Verify the result manually.
            print("Dropping you on a shell so you can try building/testing the merged source.",file=stderr)
            print("Run 'git diff HEAD~' to show the changes being merged.",file=stderr)
            print("Type 'exit' when done.",file=stderr)
            if os.path.isfile('/etc/debian_version'): # Show pull number on Debian default prompt
                os.putenv('debian_chroot',pull)
            subprocess.call([SHELL,'-i'])

        second_sha512 = tree_sha512sum()
        if first_sha512 != second_sha512:
            print("ERROR: Tree hash changed unexpectedly",file=stderr)
            sys.exit(8)

        # Retrieve PR comments and ACKs and add to commit message, store ACKs to print them with commit
        # description
        comments = retrieve_pr_comments(repo_from,pull,ghtoken) + retrieve_pr_reviews(repo_from,pull,ghtoken)
        if comments is None:
            print("ERROR: Could not fetch PR comments and reviews",file=stderr)
            sys.exit(1)
        acks = get_acks_from_comments(head_commit=head_commit, comments=comments)
        message += make_acks_message(head_commit=head_commit, acks=acks)
        # end message with SHA512 tree hash, then update message
        message += '\n\nTree-SHA512: ' + first_sha512
        try:
            subprocess.check_call([GIT,'commit','--amend','--no-gpg-sign','-m',message.encode('utf-8')])
        except subprocess.CalledProcessError:
            print("ERROR: Cannot update message.", file=stderr)
            sys.exit(4)

        # Sign the merge commit.
        print_merge_details(pull_reference, title, branch, base_branch, head_branch, acks, message)
        while True:
            reply = ask_prompt("Type 's' to sign off on the above merge, or 'x' to reject and exit.").lower()
            if reply == 's':
                try:
                    config = ['-c', 'user.name=merge-script']
                    if merge_author_email:
                        config += ['-c', f'user.email={merge_author_email}']
                    subprocess.check_call([GIT] + config + ['commit','-q','--gpg-sign','--amend','--no-edit','--reset-author'])
                    break
                except subprocess.CalledProcessError:
                    print("Error while signing, asking again.",file=stderr)
            elif reply == 'x':
                print("Not signing off on merge, exiting.",file=stderr)
                sys.exit(1)

        # Put the result in branch.
        subprocess.check_call([GIT,'checkout','-q',branch])
        subprocess.check_call([GIT,'reset','-q','--hard',local_merge_branch])
    finally:
        # Clean up temporary branches.
        subprocess.call([GIT,'checkout','-q',branch])
        subprocess.call([GIT,'branch','-q','-D',head_branch],stderr=devnull)
        subprocess.call([GIT,'branch','-q','-D',base_branch],stderr=devnull)
        subprocess.call([GIT,'branch','-q','-D',merge_branch],stderr=devnull)
        subprocess.call([GIT,'branch','-q','-D',local_merge_branch],stderr=devnull)

    # Push the result.
    while True:
        reply = ask_prompt("Type 'push' to push the result to {}, branch {}, or 'x' to exit without pushing.".format(', '.join([host_repo] + push_mirrors), branch)).lower()
        if reply == 'push':
            subprocess.check_call([GIT,'push',host_repo,'refs/heads/'+branch])
            for p_mirror in push_mirrors:
                subprocess.check_call([GIT,'push',p_mirror,'refs/heads/'+branch])
            break
        elif reply == 'x':
            sys.exit(1)

if __name__ == '__main__':
    main()