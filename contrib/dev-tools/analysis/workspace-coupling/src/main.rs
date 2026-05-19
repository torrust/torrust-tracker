//! Generates a workspace coupling report for the Torrust Tracker workspace.
//!
//! For every workspace package that has workspace-level dependencies the tool:
//!   1. Lists the declared workspace dependencies (normal / dev / build).
//!   2. Scans the package's `src/`, `tests/`, and `benches/` directories for `use DEP_MODULE::`
//!      statements and fully-qualified `DEP_MODULE::` path references, then lists the distinct
//!      top-level import paths found.
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
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::Deserialize;
use walkdir::WalkDir;

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

fn scan_imports(dirs: &[&Path], module_name: &str) -> ScanResult {
    let import_pattern = format!(r"{module_name}::[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?");
    let import_re = Regex::new(&import_pattern).expect("import regex is valid");
    let any_pattern = format!(r"\b{module_name}\b");
    let any_re = Regex::new(&any_pattern).expect("any-reference regex is valid");

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
            let Ok(content) = fs::read_to_string(entry.path()) else {
                continue;
            };

            for m in import_re.find_iter(&content) {
                result.imports.insert(m.as_str().to_owned());
            }

            if !result.has_any_reference && any_re.is_match(&content) {
                result.has_any_reference = true;
            }
        }
    }

    result
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
        "Items are extracted by scanning the package's `src/`, `tests/`, and `benches/`"
    )
    .unwrap();
    writeln!(
        out,
        "directories for `use MODULE::` statements and `MODULE::` fully-qualified path references."
    )
    .unwrap();
    writeln!(
        out,
        "The scan is text-based; it may miss items imported through re-exports or macros,"
    )
    .unwrap();
    writeln!(out, "but it is accurate enough to identify thin-dependency patterns.").unwrap();
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

fn write_dep_section(out: &mut String, dep: &Dep, scan_dirs: &[&Path]) {
    let kind = dep_kind_label(dep.kind.as_deref());
    writeln!(out, "#### `{}` [{kind}]", dep.name).unwrap();
    writeln!(out).unwrap();

    let module = crate_to_module(&dep.name);
    let scan = scan_imports(scan_dirs, &module);

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
}

fn write_coupling_details(out: &mut String, meta: &Metadata, ws_ids: &HashSet<&str>, ws_names: &HashSet<&str>) {
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
            write_dep_section(out, dep, &scan_dirs);
        }
    }
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
    writeln!(out, "- `torrust-tracker-clock` → `torrust-tracker-primitives`: only").unwrap();
    writeln!(out, "  `DurationSinceUnixEpoch` imported. Addressed by SI-02.").unwrap();
    writeln!(out, "- `torrust-tracker-configuration` → `torrust-tracker-clock`: only").unwrap();
    writeln!(out, "  `DEFAULT_TIMEOUT` imported. Addressed by SI-03.").unwrap();
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

fn generate_report(meta: &Metadata) -> String {
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
    write_coupling_details(&mut report, meta, &ws_ids, &ws_names);
    write_observations(&mut report);
    report
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    eprintln!("Running cargo metadata...");
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("failed to run cargo metadata");

    if !output.status.success() {
        eprintln!("cargo metadata failed:\n{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    let meta: Metadata = serde_json::from_slice(&output.stdout).expect("failed to parse cargo metadata JSON");

    let workspace_root = PathBuf::from(&meta.workspace_root);
    let default_output = workspace_root.join("docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md");
    let output_path: PathBuf = args.get(1).map_or(default_output, PathBuf::from);

    eprintln!("Workspace root: {}", workspace_root.display());
    eprintln!("Output file: {}", output_path.display());

    let report = generate_report(&meta);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("failed to create output directories");
    }

    fs::write(&output_path, report).expect("failed to write report file");

    eprintln!("Done.");
    eprintln!("Report: {}", output_path.display());
}
