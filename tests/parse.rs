use cargo_manifest as lib;
use cargo_manifest::Manifest;
use std::fs::read;
use std::str::FromStr;

#[test]
fn own() {
    let m = Manifest::from_slice(&read("Cargo.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("cargo-manifest", package.name);
    let m =
        Manifest::<toml::Value>::from_slice_with_metadata(&read("Cargo.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("cargo-manifest", package.name);
    assert_eq!(lib::Edition::E2018, package.edition);
}

#[test]
fn opt_level() {
    let m = Manifest::from_slice(&read("tests/opt_level.toml").unwrap()).unwrap();
    let package = m.package.as_ref().unwrap();
    assert_eq!("byteorder", package.name);
    assert_eq!(
        3,
        m.profile
            .unwrap()
            .bench
            .unwrap()
            .opt_level
            .unwrap()
            .as_integer()
            .unwrap()
    );
    assert_eq!(false, m.lib.unwrap().bench);
    assert_eq!(lib::Edition::E2015, package.edition);
    assert_eq!(1, m.patch.unwrap().len());
}

#[test]
fn autobin() {
    let m = Manifest::from_path("tests/autobin/Cargo.toml").expect("load autobin");
    let package = m.package.as_ref().unwrap();
    assert_eq!("auto-bin", package.name);
    assert_eq!(lib::Edition::E2018, package.edition);
    assert!(package.autobins);
    assert!(m.lib.is_none());
    assert_eq!(1, m.bin.as_ref().unwrap().len());
    assert_eq!(Some("auto-bin"), m.bin.unwrap()[0].name.as_deref());
}

#[test]
fn autolib() {
    let m = Manifest::from_path("tests/autolib/Cargo.toml").expect("load autolib");
    let package = m.package.as_ref().unwrap();
    assert_eq!("auto-lib", package.name);
    assert_eq!(false, package.publish);
    assert_eq!(lib::Edition::E2015, package.edition);
    assert!(package.autobins);
    assert!(!package.autoexamples);
    assert!(m.lib.is_some());
    assert_eq!("auto_lib", m.lib.unwrap().name.unwrap());
    assert_eq!(0, m.bin.unwrap().len());
}

#[test]
fn autobuild() {
    let m = Manifest::from_path("tests/autobuild/Cargo.toml").expect("load autobuild");
    let package = m.package.as_ref().unwrap();
    assert_eq!(Some(lib::Value::String("build.rs".into())), package.build);
}

#[test]
fn metadata() {
    let m = Manifest::from_path("tests/metadata/Cargo.toml").expect("load metadata");
    let package = m.package.as_ref().unwrap();
    assert_eq!("metadata", package.name);
    assert_eq!(Some(lib::Value::String("foobar.rs".into())), package.build);
}

#[test]
fn readme() {
    let base = "[package]\nname = \"foo\"\nversion = \"1\"";

    let m = Manifest::from_str(&format!("{}\nreadme = \"hello.md\"", base)).unwrap();
    let readme = m.package.unwrap().readme.unwrap();
    assert_eq!(lib::StringOrBool::String("hello.md".to_string()), readme);

    let m = Manifest::from_str(&format!("{}\nreadme = true", base)).unwrap();
    let readme = m.package.unwrap().readme.unwrap();
    assert_eq!(lib::StringOrBool::Bool(true), readme);

    let m = Manifest::from_str(&format!("{}\nreadme = 1", base));
    assert!(m.is_err());
}

#[test]
fn legacy() {
    let m = Manifest::from_slice(
        br#"[project]
                name = "foo"
                version = "1"
                "#,
    )
    .expect("parse old");
    let package = m.package.as_ref().unwrap();
    assert_eq!("foo", package.name);
    let m = Manifest::from_str("name = \"foo\"\nversion=\"1\"").expect("parse bare");
    let package = m.package.as_ref().unwrap();
    assert_eq!("foo", package.name);
}

// -- Multi-word identifiers can be specified using both snake_case and kebab-case --

/// This test ensures that the snake_case variant is handled correctly for `default-features`
#[test]
fn default_features_casing() {
    let m = Manifest::from_str(
        r#"
[package]
name = "foo"
version = "1"

[dependencies]
rusoto_core = { version = "0.45.0", default_features=false, features=["rustls"] }
"#,
    )
    .unwrap();

    let deps = m.dependencies.as_ref().unwrap();
    let rusoto_core = deps.get("rusoto_core").unwrap().detail().unwrap();
    assert!(rusoto_core.default_features.is_some());
}

/// This test ensures that the snake_case variant is handled correctly for `build-dependencies`
#[test]
fn build_dependencies_casing() {
    let m = Manifest::from_str(
        r#"
[package]
name = "foo"
version = "1"

[build_dependencies]
lazy_static = "1.4.0"
"#,
    )
    .unwrap();

    assert!(m.build_dependencies.is_some());
}

/// This test ensures that the snake_case variant is handled correctly for `dev-dependencies`
#[test]
fn dev_dependencies_casing() {
    let m = Manifest::from_str(
        r#"
[package]
name = "foo"
version = "1"

[dev_dependencies]
lazy_static = "1.4.0"
"#,
    )
    .unwrap();

    assert!(m.dev_dependencies.is_some());
}
