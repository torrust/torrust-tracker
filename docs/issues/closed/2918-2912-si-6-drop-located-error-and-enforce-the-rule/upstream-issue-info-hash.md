# Draft: remove the source location from `torrust_info_hash::ConversionError`

Repository: `torrust/torrust-info-hash` (to be opened by the maintainer). Drafted by sub-issue
SI-6 (#2918) of EPIC #2912 in `torrust/torrust-tracker`.

## Title

Remove the source location from `ConversionError`

## Body

`torrust-info-hash` 0.2.0 builds `ConversionError` with `std::panic::Location::caller()` in a
`TryFrom` impl (`info_hash.rs:273` and `:279`), so the error carries the file, line and column of
the caller and prints them in its message.

The Torrust Tracker decided that errors do not record source locations (ADR
`docs/adrs/20261118045443_errors_do_not_record_source_locations.md` in `torrust/torrust-tracker`):
the location names a path of the build machine, leaks into messages sent to clients (see
torrust/torrust-tracker#384), is not a stable contract, and the error type already identifies the
failing conversion. The tracker removed the field from all of its own error types (EPIC
torrust/torrust-tracker#2912) and added a check that fails when non-test code records a location.

Proposal:

- Remove the `location` field from `ConversionError` and the source path from its message, keeping
  the information that explains the failure (the expected and the actual length).
- This is a breaking change of a public error type: release it with a minor bump while the crate is
  0.x, and say so in the release notes.
- Replace any `#[track_caller]` on the conversion functions that existed only to feed the location.

Acceptance: no `Location::caller()` in non-test code, a test that the message contains no source
path, and a release note.
