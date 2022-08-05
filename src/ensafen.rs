//! The goal of building Rust crates safely.

use serde::{Deserialize, Serialize};
use serde_json::{self as json, Map};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;
use std::{io, str};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
struct Invocation {
    package_name: String,
    package_version: String,
    target_kind: Vec<String>,
    kind: json::Value,
    compile_mode: String,
    deps: Vec<u16>, // It's this or usize, and we probably aren't importing crates.io...?
    outputs: Vec<PathBuf>,
    links: Map<String, json::Value>,
    program: PathBuf,
    args: Vec<String>,
    env: Map<String, json::Value>,
    cwd: PathBuf,
}

fn plan_build() -> eyre::Result<Vec<Invocation>> {
    Ok(Command::new("cargo")
        .arg("build")
        .args(["-Zunstable-options", "--build-plan"])
        .output()
        .map(|io| serde_json::from_slice::<Vec<Invocation>>(&io.stdout))??)
}

fn pgx_trusted_deps() -> Result<BTreeSet<String>, io::Error> {
    Command::new("cargo")
        .arg("tree")
        .args(["--edges", "normal,build"])
        .args(["--format", r#"{p} {f}"#])
        .args(["--package", "pgx"])
        .output()
        .map(|io| {
            str::from_utf8(&io.stdout)
                .expect("Rust output will return UTF-8")
                .split("\n")
                .map(|s| s.trim_end_matches(" (*)").to_owned())
                .collect::<BTreeSet<String>>()
        })
}

fn trimmed_build_tree() -> Result<BTreeSet<String>, io::Error> {
    Command::new("cargo")
        .arg("tree")
        .args(["--edges", "normal,build"])
        .args(["--format", r#"{p} {f}"#])
        .args(["--prune", "pgx"])
        .output()
        .map(|io| {
            str::from_utf8(&io.stdout)
                .expect("Rust output will return UTF-8")
                .split("\n")
                .map(|s| s.trim_end_matches(" (*)").to_owned())
                .collect::<BTreeSet<String>>()
        })
}
