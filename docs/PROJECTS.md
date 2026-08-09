# Project routing and consumer-fixture contract

Status date: 2026-08-09

## Organization boundaries

- **Canonical source, issues, and releases:** [`flags-2-env/flags-2-env`](https://github.com/flags-2-env/flags-2-env)
- **Compatibility source through 2026-08-19 inclusive:** [`ORESoftware/flags-2-env`](https://github.com/ORESoftware/flags-2-env)
- **Organization profile and community documentation:** [`flags-2-env/.github`](https://github.com/flags-2-env/.github)
- **Consumer fixtures:** [`flags-2-env-test`](https://github.com/flags-2-env-test)
- **Linear product project:** [`github.com/flags-2-env`](https://linear.app/denman/project/githubcomflags-2-env-05db5133a267)
- **Linear fixture project:** [`Cross-Org E2E & CI Fleet`](https://linear.app/denman/project/cross-org-e2e-and-ci-fleet-71970126dfc6)
- **2026-08-08 fixture audit snapshot:** [`flags-2-env/.github#1`](https://github.com/flags-2-env/.github/issues/1)

Canonical code, tags, release artifacts, package manifests, and new
implementation issues live in `flags-2-env/flags-2-env`. The original
`ORESoftware/flags-2-env` repository keeps its issue and pull-request history
and remains a supported, commit-identical mirror through the inclusive cutoff.

## Language and feature fixtures

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

As of the status date, these 13 fixtures still declare:

```toml
[install]
dir = ".vendor/.zed"

[dependencies]
"oresoftware/flags-2-env" = "^0.2.0"
```

Those immutable compatibility pins remain supported through 2026-08-19
inclusive. The migration target for new or updated manifests is:

```toml
[install]
dir = ".vendor/.zed"

[dependencies]
"flags-2-env/flags-2-env" = "^0.3"
```

The canonical publisher declares `flags-2-env/flags-2-env@0.3.0`. Because Zed
has no package-alias field, `oresoftware/flags-2-env@0.3.0` is published as a
separate compatibility identity from the same immutable tagged commit during
the transition.

## Intentional Git/Zed interoperability

Each existing fixture pins the compatibility upstream as a gitlink at the
exact Zed materialization path:

```text
.vendor/.zed/oresoftware/flags-2-env
```

This is an intentional adopted-workspace/offline-fixture pattern:

- Git owns the exact committed checkout used by offline Docker builds.
- Zed owns package identity and version intent.
- The common fixture workflow validates that `.zpkg.toml`, `[install].dir`, `.gitmodules`, and the populated path agree.
- During the transition, the workflow proves the pin is an immutable commit shared by both source repositories.
- The workflow builds and runs the fixture and then mutates the flag contract to prove the assertions are non-vacuous.

Migrate the manifest coordinate, `.gitmodules` URL, and gitlink path together to
`.vendor/.zed/flags-2-env/flags-2-env`; do not create a second checkout path. A
future `zed overtake --git-submodules` or resolver run must preserve the single
adopted path and record immutable provenance in a resolver-generated lockfile.
Never hand-author `.zpkg.lock`.

## Additional active test repositories

The SOPS interoperability matrix is populated:

- `sops-just`
- `sops-nix`
- `sops-just-nix`
- `sops-just-nix-zpkg`

Their overlapping names describe an incremental interoperability matrix, not
duplicate product source. The organization also contains
`contract-conformance-tests`, `chaos-recovery-tests`,
`upgrade-compatibility-tests`, and `security-boundary-tests`. These are
consumer and contract evidence repositories; they do not publish the library.

## Source and release drift policy

Through 2026-08-19 inclusive:

1. Publish reviewed changes, tags, release assets, and the canonical Zed
   package from `flags-2-env/flags-2-env` first.
2. Fast-forward `ORESoftware/flags-2-env` to the exact canonical `main` object
   and publish the same tag objects without a compatibility-only merge commit.
3. Publish `oresoftware/flags-2-env` from the same tagged source with only the
   package organization overlaid.
4. Treat any Git ref or release-input divergence as a release blocker. Repair
   canonical history with a reviewed roll-forward commit and then fast-forward
   compatibility; never rewrite a public ref to manufacture parity.

After the cutoff, the original repository may remain as a read-only historical
source and redirect. Compatibility must not be removed before the end of the
inclusive support date.

## Source-of-truth boundaries

GitHub owns source, commits, submodule pins, workflows, test evidence, issues,
and release artifacts. The `github.com/flags-2-env` Linear project owns product
priority and milestones. The Cross-Org E2E & CI Fleet project owns
fixture-fleet status and cross-runtime rollout. Update both Linear projects
when the fixture set, canonical package version, gitlink pin, or activation
status changes.
