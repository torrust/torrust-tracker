# Draft: decide the future of `torrust-located-error`

Repository: `torrust/torrust-located-error` (to be opened by the maintainer). Drafted by
sub-issue SI-6 (#2918) of EPIC #2912 in `torrust/torrust-tracker`.

## Title

Decide the future of this crate: the Torrust Tracker no longer uses it

## Body

`torrust-located-error` 3.0.0 wraps an error together with the `Location::caller()` of the place
that built it. The Torrust Tracker decided that errors do not record source locations (ADR
`docs/adrs/20261118045443_errors_do_not_record_source_locations.md` in `torrust/torrust-tracker`),
removed every use (EPIC torrust/torrust-tracker#2912) and dropped the dependency in #2918. No
workspace crate of the tracker depends on it any more.

Options to decide:

1. Archive the repository and yank nothing: published versions stay resolvable for old tracker
   releases; add a README notice that the crate is unmaintained and why.
2. Keep it maintained for other users, with a README notice that the tracker does not recommend
   recording locations in errors.
3. Yank 3.0.0 (not recommended: older tracker crates that depend on it would stop resolving for new
   lock files).

Recommendation: option 1. Decide, then record the decision in this repository's README.
