//! Generates a workspace coupling report for the Torrust Tracker workspace.
//!
//! For every workspace package that has workspace-level dependencies the tool:
//!   1. Lists the declared workspace dependencies (normal / dev / build).
//!   2. Parses the package's `src/`, `tests/`, and `benches/` Rust files for `use DEP_MODULE::`
//!      statements and fully-qualified `DEP_MODULE::` path references, then lists the distinct
//!      dependency paths found.
//!
//! # Usage
//!
//! ```text
//! workspace-coupling [OUTPUT_FILE]
//! ```
//!
//! If `OUTPUT_FILE` is omitted the report is written to
//! `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`
//! relative to the workspace root.

use std::collections::{BTreeSet, HashSet};
use std::fmt::Write;
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use workspace_coupling::try_parse_imports_from_source;

const EXIT_RUNTIME_FAILURE: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;

#[derive(Serialize)]
struct CliEvent {
    kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_code: Option<u8>,
}

#[derive(Deserialize)]
struct Metadata {
    workspace_root: String,
    workspace_members: Vec<String>,
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    manifest_path: String,
    dependencies: Vec<Dep>,
}

#[derive(Deserialize)]
struct Dep {
    name: String,
    kind: Option<String>,
}

fn emit_event(event: &CliEvent) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    serde_json::to_writer(&mut stderr, event)?;
    stderr.write_all(b"\n")
}

fn emit_status(message: &str) -> io::Result<()> {
    emit_event(&CliEvent {
        kind: "status",
        message: message.to_owned(),
        detail: None,
        workspace_root: None,
        output_file: None,
        exit_code: None,
    })
}

fn emit_workspace_status(message: &str, workspace_root: &Path, output_file: &Path) -> io::Result<()> {
    emit_event(&CliEvent {
        kind: "status",
        message: message.to_owned(),
        detail: None,
        workspace_root: Some(workspace_root.display().to_string()),
        output_file: Some(output_file.display().to_string()),
        exit_code: None,
    })
}

fn emit_report_status(message: &str, output_file: &Path) -> io::Result<()> {
    emit_event(&CliEvent {
        kind: "status",
        message: message.to_owned(),
        detail: None,
        workspace_root: None,
        output_file: Some(output_file.display().to_string()),
        exit_code: None,
    })
}

fn failure(message: &str, detail: String, exit_code: u8) -> ExitCode {
    if emit_event(&CliEvent {
        kind: "error",
        message: message.to_owned(),
        detail: Some(detail),
        workspace_root: None,
        output_file: None,
        exit_code: Some(exit_code),
    })
    .is_err()
    {
        return ExitCode::FAILURE;
    }

    ExitCode::from(exit_code)
}

fn crate_to_module(name: &str) -> String {
    name.replace('-', "_")
}

fn dep_kind_label(kind: Option<&str>) -> &'static str {
    match kind {
        Some("dev") => "dev",
        Some("build") => "build",
        _ => "normal",
    }
}

fn dep_kind_order(kind: Option<&str>) -> u8 {
    match kind {
        Some("dev") => 1,
        Some("build") => 2,
        _ => 0,
    }
}

struct ScanResult {
    imports: BTreeSet<String>,
    has_any_reference: bool,
}

fn scan_imports(dirs: &[&Path], module_name: &str) -> Result<ScanResult, String> {
    let mut result = ScanResult {
        imports: BTreeSet::new(),
        has_any_reference: false,
    };

    for dir in dirs {
        if !dir.is_dir() {
            continue;
        }

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
        {
            let path = entry.path();
            let content =
                fs::read_to_string(path).map_err(|err| format!("failed to read Rust source `{}`: {err}", path.display()))?;
            let imports = try_parse_imports_from_source(&content, module_name)
                .map_err(|err| format!("failed to parse Rust source `{}`: {err}", path.display()))?;

            result.imports.extend(imports);

            if !result.has_any_reference && contains_identifier(&content, module_name) {
                result.has_any_reference = true;
            }
        }
    }

    Ok(result)
}

fn contains_identifier(source: &str, ident: &str) -> bool {
    source.match_indices(ident).any(|(start, _)| {
        let before = source[..start].chars().next_back();
        let after = source[start + ident.len()..].chars().next();

        !is_rust_identifier_char(before) && !is_rust_identifier_char(after)
    })
}

fn is_rust_identifier_char(ch: Option<char>) -> bool {
    ch.is_some_and(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn utc_timestamp() -> String {
    let output = Command::new("date").args(["-u", "+%Y-%m-%d %H:%M UTC"]).output();
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_owned(),
        _ => String::from("(timestamp unavailable)"),
    }
}

fn write_header(out: &mut String, total: usize, timestamp: &str) {
    writeln!(out, "# Workspace Coupling Report").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Generated: {timestamp}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Workspace packages: {total}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## How to read this report").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "Each section covers one workspace package that has at least one workspace-level"
    )
    .unwrap();
    writeln!(
        out,
        "dependency. For every dependency the items actually imported from it are listed:"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "- **Normal dep** — required for compilation of the library/binary.").unwrap();
    writeln!(out, "- **Dev dep** — required only in tests and benchmarks.").unwrap();
    writeln!(out, "- **Build dep** — required only in `build.rs`.").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "Items are extracted by parsing the package's `src/`, `tests/`, and `benches/`"
    )
    .unwrap();
    writeln!(
        out,
        "directories for `use MODULE::` statements and `MODULE::` fully-qualified path references."
    )
    .unwrap();
    writeln!(
        out,
        "The scan is AST-based; it may miss items generated by macros or inactive conditional code,"
    )
    .unwrap();
    writeln!(
        out,
        "but it handles normal Rust `use` forms, including groups and re-exports."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "**Signal**: a dependency with only 1–3 distinct import paths may be a candidate"
    )
    .unwrap();
    writeln!(out, "for elimination (move the item, break the edge).").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
}

fn write_leaves(out: &mut String, meta: &Metadata, ws_ids: &HashSet<&str>, ws_names: &HashSet<&str>) {
    writeln!(out, "## Packages with no workspace dependencies").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "These packages are leaves (no workspace dep) and are prime extraction candidates."
    )
    .unwrap();
    writeln!(out).unwrap();

    let mut leaf_names: BTreeSet<&str> = BTreeSet::new();
    for pkg in &meta.packages {
        if !ws_ids.contains(pkg.id.as_str()) {
            continue;
        }
        let ws_dep_count = pkg.dependencies.iter().filter(|d| ws_names.contains(d.name.as_str())).count();
        if ws_dep_count == 0 {
            leaf_names.insert(&pkg.name);
        }
    }

    if leaf_names.is_empty() {
        writeln!(out, "_None._").unwrap();
    } else {
        for name in &leaf_names {
            writeln!(out, "- `{name}`").unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
}

fn write_dep_section(out: &mut String, dep: &Dep, scan_dirs: &[&Path]) -> Result<(), String> {
    let kind = dep_kind_label(dep.kind.as_deref());
    writeln!(out, "#### `{}` [{kind}]", dep.name).unwrap();
    writeln!(out).unwrap();

    let module = crate_to_module(&dep.name);
    let scan = scan_imports(scan_dirs, &module)?;

    if !scan.imports.is_empty() {
        for import in &scan.imports {
            writeln!(out, "- `{import}`").unwrap();
        }
    } else if scan.has_any_reference {
        writeln!(
            out,
            "_Items not extracted — dependency used without a direct `use` path (macro, re-export, or glob import)._"
        )
        .unwrap();
    } else if scan_dirs.iter().any(|d| d.is_dir()) {
        writeln!(
            out,
            "_No `{module}::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._"
        )
        .unwrap();
    } else {
        writeln!(out, "_Source directories not found._").unwrap();
    }

    writeln!(out).unwrap();
    Ok(())
}

fn write_coupling_details(
    out: &mut String,
    meta: &Metadata,
    ws_ids: &HashSet<&str>,
    ws_names: &HashSet<&str>,
) -> Result<(), String> {
    writeln!(out, "## Package coupling details").unwrap();
    writeln!(out).unwrap();

    let mut sorted_packages: Vec<&Package> = meta.packages.iter().filter(|p| ws_ids.contains(p.id.as_str())).collect();
    sorted_packages.sort_by(|a, b| a.name.cmp(&b.name));

    for pkg in sorted_packages {
        let manifest_dir = Path::new(&pkg.manifest_path)
            .parent()
            .expect("manifest path has a parent directory");
        let src_dir = manifest_dir.join("src");
        let tests_dir = manifest_dir.join("tests");
        let benches_dir = manifest_dir.join("benches");
        let scan_dirs = [src_dir.as_path(), tests_dir.as_path(), benches_dir.as_path()];

        let mut ws_deps: Vec<&Dep> = pkg
            .dependencies
            .iter()
            .filter(|d| ws_names.contains(d.name.as_str()))
            .collect();

        if ws_deps.is_empty() {
            continue;
        }

        ws_deps.sort_by(|a, b| {
            dep_kind_order(a.kind.as_deref())
                .cmp(&dep_kind_order(b.kind.as_deref()))
                .then(a.name.cmp(&b.name))
        });

        writeln!(out, "### `{}`", pkg.name).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Workspace deps: {}", ws_deps.len()).unwrap();
        writeln!(out).unwrap();

        for dep in ws_deps {
            write_dep_section(out, dep, &scan_dirs)?;
        }
    }

    Ok(())
}

fn write_observations(out: &mut String) {
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Observations").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "To be filled in after reviewing the report above.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "### Known thin dependencies (pre-existing)").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "None — previously known thin dependencies have been resolved:").unwrap();
    writeln!(out, "- `torrust-clock` → `torrust-tracker-primitives` (resolved by SI-02)").unwrap();
    writeln!(out, "- `torrust-tracker-configuration` → `torrust-clock` (resolved by SI-03)").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "### New findings").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "Record any new thin-dependency or cluster-dependency findings here, with a"
    )
    .unwrap();
    writeln!(out, "reference to the subissue opened for each.").unwrap();
}

fn generate_report(meta: &Metadata) -> Result<String, String> {
    let ws_ids: HashSet<&str> = meta.workspace_members.iter().map(String::as_str).collect();
    let ws_names: HashSet<&str> = meta
        .packages
        .iter()
        .filter(|p| ws_ids.contains(p.id.as_str()))
        .map(|p| p.name.as_str())
        .collect();
    let total = ws_names.len();
    let timestamp = utc_timestamp();

    let mut report = String::new();
    write_header(&mut report, total, &timestamp);
    write_leaves(&mut report, meta, &ws_ids, &ws_names);
    write_coupling_details(&mut report, meta, &ws_ids, &ws_names)?;
    write_observations(&mut report);
    Ok(report)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        return failure(
            "invalid arguments",
            format!("expected at most one output file argument, got {}", args.len() - 1),
            EXIT_USAGE_ERROR,
        );
    }

    if emit_status("running cargo metadata").is_err() {
        return ExitCode::FAILURE;
    }

    let output = match Command::new("cargo").args(["metadata", "--format-version", "1"]).output() {
        Ok(output) => output,
        Err(err) => {
            return failure("failed to run cargo metadata", err.to_string(), EXIT_RUNTIME_FAILURE);
        }
    };

    if !output.status.success() {
        return failure(
            "cargo metadata failed",
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            EXIT_RUNTIME_FAILURE,
        );
    }

    let meta: Metadata = match serde_json::from_slice(&output.stdout) {
        Ok(meta) => meta,
        Err(err) => {
            return failure("failed to parse cargo metadata JSON", err.to_string(), EXIT_RUNTIME_FAILURE);
        }
    };

    let workspace_root = PathBuf::from(&meta.workspace_root);
    let default_output = workspace_root.join("docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md");
    let output_path: PathBuf = args.get(1).map_or(default_output, PathBuf::from);

    if emit_workspace_status("workspace resolved", &workspace_root, &output_path).is_err() {
        return ExitCode::FAILURE;
    }

    let report = match generate_report(&meta) {
        Ok(report) => report,
        Err(err) => return failure("failed to generate report", err, EXIT_RUNTIME_FAILURE),
    };

    if let Some(parent) = output_path.parent()
        && let Err(err) = fs::create_dir_all(parent)
    {
        return failure(
            "failed to create output directories",
            format!("{}: {err}", parent.display()),
            EXIT_RUNTIME_FAILURE,
        );
    }

    if let Err(err) = fs::write(&output_path, report) {
        return failure(
            "failed to write report file",
            format!("{}: {err}", output_path.display()),
            EXIT_RUNTIME_FAILURE,
        );
    }

    if emit_report_status("report written", &output_path).is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
