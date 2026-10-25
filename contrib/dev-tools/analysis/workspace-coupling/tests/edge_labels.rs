use std::collections::HashSet;

use serde_json::{Value, json};
use workspace_coupling::{Dep, Package};

fn dep(name: &str, kind: Option<&str>, optional: bool) -> Value {
    json!({ "name": name, "kind": kind, "optional": optional })
}

fn package(name: &str, dependencies: &[Value], features: &Value) -> Package {
    serde_json::from_value(json!({
        "id": format!("path+file:///workspace/{name}#0.1.0"),
        "name": name,
        "manifest_path": format!("/workspace/{name}/Cargo.toml"),
        "dependencies": dependencies,
        "features": features,
    }))
    .expect("valid cargo metadata package")
}

fn label_of(package: &Package, dependency: &str) -> String {
    let dep: &Dep = package
        .dependencies
        .iter()
        .find(|dep| dep.name == dependency)
        .expect("the package declares the dependency");

    package.edge_kind(dep).to_string()
}

#[test]
fn it_should_skip_a_self_edge_when_listing_workspace_dependencies() {
    let ws_names = HashSet::from(["a", "c"]);
    let a = package(
        "a",
        &[dep("a", Some("dev"), false), dep("c", None, false), dep("serde", None, false)],
        &json!({ "testing": [] }),
    );

    let names: Vec<&str> = a
        .workspace_dependencies(&ws_names)
        .iter()
        .map(|dep| dep.name.as_str())
        .collect();

    assert_eq!(names, ["c"]);
}

#[test]
fn it_should_list_no_workspace_dependency_when_the_only_one_is_a_self_edge() {
    let ws_names = HashSet::from(["a"]);
    let a = package("a", &[dep("a", Some("dev"), false)], &json!({ "testing": [] }));

    assert!(a.workspace_dependencies(&ws_names).is_empty());
}

#[test]
fn it_should_label_an_optional_dependency_enabled_only_by_the_testing_feature_as_optional_testing() {
    let a = package("a", &[dep("c", None, true)], &json!({ "testing": ["dep:c"] }));

    assert_eq!(label_of(&a, "c"), "optional (testing)");
}

#[test]
fn it_should_label_an_optional_dependency_enabled_by_the_implicit_feature_syntax_as_optional_testing() {
    // Cargo adds the implicit feature `c = ["dep:c"]` for an optional dependency.
    let a = package("a", &[dep("c", None, true)], &json!({ "c": ["dep:c"], "testing": ["c"] }));

    assert_eq!(label_of(&a, "c"), "optional (testing)");
}

#[test]
fn it_should_label_an_optional_dependency_that_only_its_implicit_feature_enables_as_optional() {
    let a = package("a", &[dep("c", None, true)], &json!({ "c": ["dep:c"], "testing": [] }));

    assert_eq!(label_of(&a, "c"), "optional");
}

#[test]
fn it_should_label_an_optional_dependency_enabled_by_another_feature_with_that_feature() {
    let a = package("a", &[dep("c", None, true)], &json!({ "metrics": ["dep:c"] }));

    assert_eq!(label_of(&a, "c"), "optional (metrics)");
}

#[test]
fn it_should_join_every_enabling_feature_when_several_enable_an_optional_dependency() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "testing": ["dep:c"], "metrics": ["dep:c"], "unrelated": ["dep:other"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (metrics, testing)");
}

#[test]
fn it_should_not_label_the_dependency_as_testing_only_when_another_feature_also_enables_it() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "testing": ["dep:c"], "extras": ["c/some-feature"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (extras, testing)");
}

#[test]
fn it_should_not_label_the_dependency_as_testing_only_when_the_default_feature_enables_testing() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "default": ["testing"], "testing": ["dep:c"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (default, testing)");
}

#[test]
fn it_should_follow_a_chain_of_feature_aliases_to_the_dependency() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "default": ["tools"], "tools": ["extras"], "extras": ["testing"], "testing": ["dep:c"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (default, extras, testing, tools)");
}

#[test]
fn it_should_follow_the_implicit_feature_of_the_dependency_through_an_alias() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "c": ["dep:c"], "default": ["testing"], "testing": ["c"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (default, testing)");
}

#[test]
fn it_should_terminate_when_features_enable_each_other() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "testing": ["dep:c", "extras"], "extras": ["testing"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (extras, testing)");
}

#[test]
fn it_should_not_follow_a_feature_that_only_enables_a_feature_of_another_dependency() {
    let a = package(
        "a",
        &[dep("c", None, true)],
        &json!({ "testing": ["dep:c"], "extras": ["other/testing", "other?/testing"] }),
    );

    assert_eq!(label_of(&a, "c"), "optional (testing)");
}

#[test]
fn it_should_label_an_optional_dependency_that_no_feature_enables_as_optional() {
    let a = package("a", &[dep("c", None, true)], &json!({ "testing": [] }));

    assert_eq!(label_of(&a, "c"), "optional");
}

#[test]
fn it_should_match_the_enabling_feature_by_the_dependency_key_when_the_dependency_is_renamed() {
    let renamed = json!({ "name": "c", "kind": null, "optional": true, "rename": "c-alias" });
    let a = package("a", &[renamed], &json!({ "testing": ["dep:c-alias"] }));

    assert_eq!(label_of(&a, "c"), "optional (testing)");
}

#[test]
fn it_should_not_treat_a_weak_dependency_feature_as_enabling_the_dependency() {
    let a = package("a", &[dep("c", None, true)], &json!({ "extras": ["c?/some-feature"] }));

    assert_eq!(label_of(&a, "c"), "optional");
}

#[test]
fn it_should_keep_the_normal_label_for_a_required_normal_dependency() {
    let a = package("a", &[dep("c", None, false)], &json!({ "testing": [] }));

    assert_eq!(label_of(&a, "c"), "normal");
}

#[test]
fn it_should_keep_the_dev_and_build_labels() {
    let a = package(
        "a",
        &[dep("c", Some("dev"), false), dep("d", Some("build"), false)],
        &json!({ "testing": [] }),
    );

    assert_eq!(label_of(&a, "c"), "dev");
    assert_eq!(label_of(&a, "d"), "build");
}

#[test]
fn it_should_read_metadata_that_has_neither_the_optional_field_nor_the_features_table() {
    let a: Package = serde_json::from_value(json!({
        "id": "path+file:///workspace/a#0.1.0",
        "name": "a",
        "manifest_path": "/workspace/a/Cargo.toml",
        "dependencies": [{ "name": "c", "kind": null }],
    }))
    .expect("valid cargo metadata package");

    assert_eq!(label_of(&a, "c"), "normal");
}

#[test]
fn it_should_order_the_edge_kinds_normal_then_optional_then_dev_then_build() {
    let a = package(
        "a",
        &[
            dep("build", Some("build"), false),
            dep("dev", Some("dev"), false),
            dep("opt", None, true),
            dep("normal", None, false),
        ],
        &json!({ "testing": ["dep:opt"] }),
    );

    let mut dependencies: Vec<&Dep> = a.dependencies.iter().collect();
    dependencies.sort_by_key(|dep| a.edge_kind(dep));

    let names: Vec<&str> = dependencies.iter().map(|dep| dep.name.as_str()).collect();
    assert_eq!(names, ["normal", "opt", "dev", "build"]);
}
