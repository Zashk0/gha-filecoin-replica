# gha-filecoin-replica

Authorized PoC for an Immunefi submission against Filecoin.

This repo mirrors the workflow shape of `filecoin-project/rust-fil-proofs/.github/workflows/ci.yml` (PR-triggered on `[self-hosted, linux, x64, 4xlarge]`, runs cargo). The "secret" leaked here is a fake marker value set on this repo only.

The runner used by the public PoC is GitHub-hosted ephemeral. The same primitive against Filecoin's `i-03b76a324dcf70ab0` and the rest of the org-shared AWS EC2 fleet (`i-089c6077c3e58d529` on bellperson, `i-00048b1f22da5ab2b` on boost, `i-0c00f1428d07f6896` on rust-filecoin-proofs-api) writes to a persistent host shared across multiple Filecoin org repos.

## How to read this PoC

- `master` branch contains the workflow shape under test
- `attacker-poc` branch adds a `build.rs` that runs during `cargo test --release` and dumps host info plus the `RUNNER_PRETEND_SECRET` env var via reverse + hex encoding
- PR #1 from `attacker-poc` triggered the CI workflow

## What you should see in the run log

```
[BUILDRS:whoami] runner
[BUILDRS:hostname] runnervmeorf1
[BUILDRS:ls-runner] ... .cargo .docker .rustup actions-runner work
[BUILDRS:curl-egress] Responsive is better than fast.
[BUILDRS:secret-len] 41
[BUILDRS:secret-rev] EFAC-FEEBDAED-NIOCELIF-terces-rennur-ekaf
[BUILDRS:secret-hex] 66616b652d72756e6e65722d7365637265742d46494c45434f494e2d44454144424545462d43414645
```

Reversed: `fake-runner-secret-FILECOIN-DEADBEEF-CAFE`. Hex decodes to the same.

## Why this matters for Filecoin

The Filecoin org runs four scoped repos (rust-fil-proofs, bellperson, boost, rust-filecoin-proofs-api) on a shared self-hosted AWS EC2 runner pool, all in the `ip-10-0-10X.X` private subnet. Public run logs across the four repos show four distinct EC2 instance IDs all assigning runs from the same pool. A PR to any one of these repos lands a job on a runner instance from this fleet. A malicious Cargo build script gets RCE on that instance. Persistence on the instance survives across subsequent jobs from any of the four repos.

This is the textbook self-hosted runner takeover described in Adnan Khan's Praetorian writeup and the gato-x toolkit. The same primitive that this PoC validates (cargo build script RCE from PR HEAD) applies directly.

## Authorization

This is a researcher-owned private-then-public repo. No production Filecoin infrastructure was touched. The fake marker secret is unique to this repo and has no value outside the demonstration.
