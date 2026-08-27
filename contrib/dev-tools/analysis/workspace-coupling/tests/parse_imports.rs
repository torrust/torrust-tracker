use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use workspace_coupling::parse_imports_from_source;

fn expected_imports(imports: &[&str]) -> BTreeSet<String> {
    imports.iter().map(ToString::to_string).collect()
}

#[test]
fn parses_brace_import_groups() {
    let source = r"
        use torrust_tracker_contrib_bencode::{BMutAccess, ben_int, ben_map};
    ";

    assert_eq!(
        parse_imports_from_source(source, "torrust_tracker_contrib_bencode"),
        expected_imports(&[
            "torrust_tracker_contrib_bencode::BMutAccess",
            "torrust_tracker_contrib_bencode::ben_int",
            "torrust_tracker_contrib_bencode::ben_map",
        ])
    );
}

#[test]
fn parses_pub_use_reexports() {
    let source = r"
        pub use bittorrent_peer_id::{PeerClient, PeerId};
    ";

    assert_eq!(
        parse_imports_from_source(source, "bittorrent_peer_id"),
        expected_imports(&["bittorrent_peer_id::PeerClient", "bittorrent_peer_id::PeerId"])
    );
}

#[test]
fn parses_nested_aliased_and_glob_imports() {
    let source = r"
        use a::b::{c, d as e};
        use a::*;
    ";

    assert_eq!(
        parse_imports_from_source(source, "a"),
        expected_imports(&["a::*", "a::b::c", "a::b::d"])
    );
}

#[test]
fn parses_root_aliased_imports() {
    let source = r"
        use torrust_tracker_configuration as configuration;
    ";

    assert_eq!(
        parse_imports_from_source(source, "torrust_tracker_configuration"),
        expected_imports(&["torrust_tracker_configuration"])
    );
}

#[test]
fn parses_fully_qualified_path_references() {
    let source = r"
        fn build() {
            let _ = dep_crate::nested::Thing::new();
        }
    ";

    assert_eq!(
        parse_imports_from_source(source, "dep_crate"),
        expected_imports(&["dep_crate::nested::Thing"])
    );
}

#[test]
fn parses_fully_qualified_path_references_inside_macros() {
    let source = r"
        fn build() -> bool {
            matches!(dep_crate::Thing::A, dep_crate::Thing::A)
        }
    ";

    assert_eq!(
        parse_imports_from_source(source, "dep_crate"),
        expected_imports(&["dep_crate::Thing::A"])
    );
}

#[test]
fn returns_empty_set_when_module_is_not_referenced() {
    let source = r"
        use other_crate::Thing;

        fn build() -> other_crate::Thing {
            other_crate::Thing
        }
    ";

    assert!(parse_imports_from_source(source, "dep_crate").is_empty());
}

#[test]
fn binary_extracts_grouped_reexported_aliased_and_glob_imports() {
    let workspace = FixtureWorkspace::new("valid");
    write_workspace(
        &workspace.root,
        &[
            "bittorrent-peer-id",
            "torrust-tracker-configuration",
            "torrust-tracker-contrib-bencode",
            "torrust-tracker-located-error",
        ],
        r"
            use torrust_tracker_contrib_bencode::{BMutAccess, ben_int, ben_map};
            use torrust_tracker_located_error::{DynError, Located, LocatedError};
            use torrust_tracker_configuration::v3_0_0::{core::Core, udp_tracker::UdpTracker};
            use torrust_tracker_configuration::*;
            use torrust_tracker_configuration as configuration;
            pub use bittorrent_peer_id::{PeerClient, PeerId};
            use bittorrent_peer_id::client::{ClientKind as Kind, identify};

            fn checks_mode() -> bool {
                matches!(torrust_tracker_configuration::Mode::Strict, _)
            }
        ",
    );

    let output_path = workspace.root.join("report.md");
    let output = Command::new(workspace_coupling_binary())
        .arg(&output_path)
        .current_dir(&workspace.root)
        .output()
        .expect("failed to run workspace-coupling binary");

    assert!(
        output.status.success(),
        "workspace-coupling failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"");

    assert_stderr_is_ndjson(&output.stderr);

    let report = fs::read_to_string(output_path).expect("failed to read generated report");
    for import in [
        "bittorrent_peer_id::PeerClient",
        "bittorrent_peer_id::PeerId",
        "bittorrent_peer_id::client::ClientKind",
        "bittorrent_peer_id::client::identify",
        "torrust_tracker_configuration",
        "torrust_tracker_configuration::*",
        "torrust_tracker_configuration::v3_0_0::core::Core",
        "torrust_tracker_configuration::Mode::Strict",
        "torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker",
        "torrust_tracker_contrib_bencode::BMutAccess",
        "torrust_tracker_contrib_bencode::ben_int",
        "torrust_tracker_contrib_bencode::ben_map",
        "torrust_tracker_located_error::DynError",
        "torrust_tracker_located_error::Located",
        "torrust_tracker_located_error::LocatedError",
    ] {
        assert!(
            report.contains(&format!("- `{import}`")),
            "missing import `{import}` in report:\n{report}"
        );
    }

    assert!(!report.contains("Items not extracted"));
}

#[test]
fn binary_reports_malformed_rust_as_json_error() {
    let workspace = FixtureWorkspace::new("malformed");
    write_workspace(
        &workspace.root,
        &["dep-crate"],
        r"
            use dep_crate::{Alpha,;
        ",
    );

    let output_path = workspace.root.join("report.md");
    let output = Command::new(workspace_coupling_binary())
        .arg(output_path)
        .current_dir(&workspace.root)
        .output()
        .expect("failed to run workspace-coupling binary");

    assert!(!output.status.success());
    assert_eq!(output.stdout, b"");

    let events = assert_stderr_is_ndjson(&output.stderr);
    assert!(events.iter().any(|event| {
        event["kind"] == "error"
            && event["message"] == "failed to generate report"
            && event["exit_code"] == 1
            && event["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("failed to parse Rust source"))
    }));
}

struct FixtureWorkspace {
    root: PathBuf,
}

impl FixtureWorkspace {
    fn new(name: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("workspace-coupling-{name}-{}-{timestamp}", std::process::id()));

        fs::create_dir_all(&root).expect("failed to create fixture workspace");

        Self { root }
    }
}

impl Drop for FixtureWorkspace {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

fn write_workspace(root: &Path, dependency_names: &[&str], consumer_source: &str) {
    let members = dependency_names
        .iter()
        .copied()
        .chain(["consumer"])
        .map(|member| format!("\"{member}\""))
        .collect::<Vec<_>>()
        .join(", ");
    write_file(
        root,
        "Cargo.toml",
        &format!(
            r#"
                [workspace]
                members = [{members}]
                resolver = "3"
            "#
        ),
    );

    for dependency_name in dependency_names {
        write_package(root, dependency_name, None, "pub struct Placeholder;");
    }

    let dependencies = dependency_names
        .iter()
        .map(|dependency_name| format!("{dependency_name} = {{ path = \"../{dependency_name}\" }}"))
        .collect::<Vec<_>>()
        .join("\n");
    write_package(root, "consumer", Some(&dependencies), consumer_source);
}

fn write_package(root: &Path, package_name: &str, dependencies: Option<&str>, source: &str) {
    let dependency_section = dependencies.map_or_else(String::new, |dependencies| format!("\n[dependencies]\n{dependencies}\n"));
    write_file(
        root,
        &format!("{package_name}/Cargo.toml"),
        &format!(
            r#"
                [package]
                name = "{package_name}"
                version = "0.1.0"
                edition = "2024"
                publish = false
                {dependency_section}
            "#
        ),
    );
    write_file(root, &format!("{package_name}/src/lib.rs"), source);
}

fn write_file(root: &Path, relative_path: &str, contents: &str) {
    let path = root.join(relative_path);
    let parent = path.parent().expect("fixture path has a parent");
    fs::create_dir_all(parent).expect("failed to create fixture parent directory");
    fs::write(path, contents).expect("failed to write fixture file");
}

fn workspace_coupling_binary() -> PathBuf {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_workspace-coupling") {
        return path.into();
    }

    if let Some(path) = option_env!("CARGO_BIN_EXE_workspace-coupling") {
        let path = PathBuf::from(path);
        if path.exists() {
            return path;
        }
    }

    let current_exe = std::env::current_exe().expect("failed to determine current test executable path");
    let profile_dir = current_exe
        .parent()
        .and_then(Path::parent)
        .expect("failed to determine Cargo profile directory from test executable path");

    let mut candidate = profile_dir.join("workspace-coupling");
    if cfg!(windows) {
        candidate.set_extension("exe");
    }

    assert!(
        candidate.exists(),
        "workspace-coupling binary not found at {}",
        candidate.display()
    );
    candidate
}

fn assert_stderr_is_ndjson(stderr: &[u8]) -> Vec<Value> {
    let stderr = std::str::from_utf8(stderr).expect("stderr is not valid UTF-8");
    assert!(!stderr.trim().is_empty(), "stderr should contain NDJSON events");

    stderr
        .lines()
        .map(|line| serde_json::from_str(line).expect("stderr line is not valid JSON"))
        .collect()
}
