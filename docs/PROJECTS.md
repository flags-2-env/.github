# Project routing and consumer-fixture contract

Status date: 2026-08-08

## Organization boundaries

- **Canonical source and releases:** [`ORESoftware/flags-2-env`](https://github.com/ORESoftware/flags-2-env)
- **Name-holder and community documentation:** [`flags-2-env/.github`](https://github.com/flags-2-env/.github)
- **Consumer fixtures:** [`flags-2-env-test`](https://github.com/flags-2-env-test)
- **Linear product project:** [`github.com/flags-2-env`](https://linear.app/denman/project/githubcomflags-2-env-05db5133a267)
- **Linear fixture project:** [`Cross-Org E2E & CI Fleet`](https://linear.app/denman/project/cross-org-e2e-and-ci-fleet-71970126dfc6)
- **Audit issue:** [`flags-2-env/.github#1`](https://github.com/flags-2-env/.github/issues/1)

The `flags-2-env` organization must not accumulate product source. Canonical code, tags, release artifacts, package manifests, and implementation issues live in `ORESoftware/flags-2-env`.

## Populated test fleet

The following 13 repositories are active consumer fixtures:

| Repository | Runtime or purpose |
| --- | --- |
| `rust-app` | Rust / `cc` integration |
| `cpp-app` | C++17 / header client |
| `java-app` | Java / JNI |
| `ruby-app` | Ruby / Fiddle FFI |
| `nodejs-app` | Node.js native addon |
| `bun-app` | Bun FFI |
| `deno-app` | Deno FFI |
| `dart-app` | Dart FFI |
| `golang-app` | Go / cgo |
| `python-app` | Python / ctypes |
| `php-app` | PHP FFI |
| `gleam-app` | Gleam / Erlang NIF |
| `feature-matrix` | Cross-runtime feature coverage |

Every populated fixture declares exactly:

```toml
[install]
dir = ".vendor/.zed"

[dependencies]
"oresoftware/flags-2-env" = "^0.2.0"
```

The canonical publisher itself declares package identity `oresoftware/flags-2-env` at version `0.2.0`.

## Intentional Git/Zed interoperability

Each fixture pins the canonical upstream as a gitlink at the exact Zed materialization path:

```text
.vendor/.zed/oresoftware/flags-2-env
```

This is an intentional adopted-workspace/offline-fixture pattern:

- Git owns the exact committed checkout used by offline Docker builds.
- Zed owns package identity and version intent.
- The common fixture workflow validates that `.zpkg.toml`, `[install].dir`, `.gitmodules`, and the populated path agree.
- The workflow proves the pin is an immutable commit published by `ORESoftware/flags-2-env`.
- The workflow builds and runs the fixture and then mutates the flag contract to prove the assertions are non-vacuous.

Do not create another checkout path. A future `zed overtake --git-submodules` or resolver run must preserve this same path and record immutable provenance in a resolver-generated lockfile. Never hand-author `.zpkg.lock`.

## Empty staging repositories

The following repositories currently contain no source or manifest:

- `sops-just`
- `sops-nix`
- `sops-just-nix`
- `sops-just-nix-zpkg`

Their overlapping names describe a planned incremental interoperability matrix; they are not considered redundant merely by naming. Do not create package identities or placeholder manifests until each repository has an explicit test contract, implementation, and ownership boundary.

## Source-of-truth boundaries

GitHub owns source, commits, submodule pins, workflows, test evidence, issues, and release artifacts. The `github.com/flags-2-env` Linear project owns product priority and milestones. The Cross-Org E2E & CI Fleet project owns fixture-fleet status and cross-runtime rollout. Update both Linear projects when the fixture set, canonical package version, gitlink pin, or activation status changes.
