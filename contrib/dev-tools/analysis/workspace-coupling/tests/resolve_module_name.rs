use serde_json::json;
use workspace_coupling::{Dep, Package, resolve_module_name};

fn dep(name: &str, rename: Option<&str>) -> Dep {
    serde_json::from_value(json!({ "name": name, "kind": null, "rename": rename })).expect("valid cargo metadata dependency")
}

fn package(name: &str, targets: &serde_json::Value) -> Package {
    serde_json::from_value(json!({
        "id": format!("path+file:///workspace/{name}#0.1.0"),
        "name": name,
        "manifest_path": format!("/workspace/{name}/Cargo.toml"),
        "dependencies": [],
        "targets": targets,
    }))
    .expect("valid cargo metadata package")
}

#[test]
fn it_should_use_the_package_name_when_the_dependency_is_not_renamed_and_has_no_custom_library_name() {
    let dep_package = package(
        "torrust-tracker-primitives",
        &json!([{ "name": "torrust-tracker-primitives", "kind": ["lib"] }]),
    );

    assert_eq!(
        resolve_module_name(&dep("torrust-tracker-primitives", None), Some(&dep_package)),
        "torrust_tracker_primitives"
    );
}

#[test]
fn it_should_use_the_package_name_when_the_dependency_package_is_unknown() {
    assert_eq!(resolve_module_name(&dep("some-package", None), None), "some_package");
}

#[test]
fn it_should_use_the_rename_when_the_dependency_is_renamed_in_the_dependent_manifest() {
    let dep_package = package(
        "torrust-tracker-client-lib",
        &json!([{ "name": "torrust-tracker-client-lib", "kind": ["lib"] }]),
    );

    assert_eq!(
        resolve_module_name(
            &dep("torrust-tracker-client-lib", Some("torrust-tracker-client")),
            Some(&dep_package)
        ),
        "torrust_tracker_client"
    );
}

#[test]
fn it_should_prefer_the_rename_over_a_custom_library_name() {
    let dep_package = package("b", &json!([{ "name": "custom_lib", "kind": ["lib"] }]));

    assert_eq!(resolve_module_name(&dep("b", Some("foo")), Some(&dep_package)), "foo");
}

#[test]
fn it_should_use_the_library_target_name_when_the_library_has_a_custom_name() {
    let dep_package = package(
        "torrust-tracker-client-lib",
        &json!([
            { "name": "tracker_client", "kind": ["bin"] },
            { "name": "torrust_tracker_client", "kind": ["lib"] },
        ]),
    );

    assert_eq!(
        resolve_module_name(&dep("torrust-tracker-client-lib", None), Some(&dep_package)),
        "torrust_tracker_client"
    );
}

#[test]
fn it_should_replace_hyphens_in_the_library_target_name_and_in_the_rename() {
    let dep_package = package("b", &json!([{ "name": "my-lib", "kind": ["lib"] }]));

    assert_eq!(resolve_module_name(&dep("b", None), Some(&dep_package)), "my_lib");
    assert_eq!(
        resolve_module_name(&dep("b", Some("my-alias")), Some(&dep_package)),
        "my_alias"
    );
}

#[test]
fn it_should_use_the_name_of_a_proc_macro_target() {
    let dep_package = package("b-derive", &json!([{ "name": "b_macros", "kind": ["proc-macro"] }]));

    assert_eq!(resolve_module_name(&dep("b-derive", None), Some(&dep_package)), "b_macros");
}

#[test]
fn it_should_ignore_targets_that_other_crates_cannot_import() {
    let dep_package = package(
        "b",
        &json!([
            { "name": "b_c", "kind": ["cdylib"] },
            { "name": "b_s", "kind": ["staticlib"] },
        ]),
    );

    assert_eq!(resolve_module_name(&dep("b", None), Some(&dep_package)), "b");
}

#[test]
fn it_should_resolve_the_same_package_differently_for_each_dependent() {
    let dep_package = package("b", &json!([{ "name": "b", "kind": ["lib"] }]));

    assert_eq!(resolve_module_name(&dep("b", Some("foo")), Some(&dep_package)), "foo");
    assert_eq!(resolve_module_name(&dep("b", Some("bar")), Some(&dep_package)), "bar");
    assert_eq!(resolve_module_name(&dep("b", None), Some(&dep_package)), "b");
}

#[test]
fn it_should_use_the_name_of_an_rlib_or_dylib_target() {
    let rlib_package = package("b", &json!([{ "name": "b_rlib", "kind": ["rlib"] }]));
    let dylib_package = package("b", &json!([{ "name": "b_dylib", "kind": ["dylib"] }]));

    assert_eq!(resolve_module_name(&dep("b", None), Some(&rlib_package)), "b_rlib");
    assert_eq!(resolve_module_name(&dep("b", None), Some(&dylib_package)), "b_dylib");
}

#[test]
fn it_should_use_the_library_target_name_when_the_target_has_several_kinds() {
    let dep_package = package("b", &json!([{ "name": "b_multi", "kind": ["cdylib", "rlib"] }]));

    assert_eq!(resolve_module_name(&dep("b", None), Some(&dep_package)), "b_multi");
}
