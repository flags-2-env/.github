# Organization context

Three GitHub organizations carry the `flags-2-env` project. They are not
mirrors of each other, and only one of them holds source code.

## `ORESoftware` — canonical

<https://github.com/ORESoftware/flags-2-env>

Everything is here: the C core (`src/parser.c`), the per-language clients under
`clients/`, the CLI, the test suite, the release automation, and the issue
tracker. Clone this. File issues here. Open pull requests here.

Package identities published from it:

| Registry | Identity |
| --- | --- |
| npm | `@oresoftware/f2e` |
| crates.io | `flags2env` |
| PyPI | `flags2env` |
| Maven Central | `com.oresoftware:flags2env` |
| pub.dev | `flags2env` |
| Hex | `flags2env` |
| zed-pkg | `oresoftware/flags-2-env` |

## `flags-2-env` — this org

Name reservation plus org-wide community health files. It exists so the obvious
URL for the project resolves to a page that redirects readers to the canonical
repository instead of to a 404 or to someone else's squat.

It should never accumulate library source. If it does, that code is stale by
definition, because releases are cut from `ORESoftware/flags-2-env`.

## `flags-2-env-test` — consumer fixtures

<https://github.com/flags-2-env-test>

One repository per language runtime. Each one is a small application that
declares its flags in `.cli-flags.toml`, pulls `oresoftware/flags-2-env` in as a
dependency (declared in `.zpkg.toml`, materialized today by a pinned git
submodule at `.vendor/.zed/oresoftware/flags-2-env`), and runs in Docker to
prove the parse result is identical across runtimes.

These repos are fixtures, not products. They are deliberately small, they pin
the upstream commit they were verified against, and they are allowed to break
loudly when the contract changes — that is the entire point.

## Verified fleet state — 2026-08-08

The 12 language fixtures and `feature-matrix` all declare only the canonical
`oresoftware/flags-2-env = "^0.2.0"` dependency and materialize under
`.vendor/.zed`. Their gitlinks use that exact materialization path, making the
committed checkout an intentional offline stand-in rather than an accidental
second source tree.

The common fixture workflow verifies the manifest/path relationship, confirms
the pinned SHA belongs to the canonical upstream, runs the Docker contract,
and deliberately mutates the flag contract to prove the assertions can fail.

Four additional repositories — `sops-just`, `sops-nix`, `sops-just-nix`, and
`sops-just-nix-zpkg` — are empty staging nodes. Do not fabricate manifests or
package identities until their individual interoperability contracts exist.
Their names describe a planned incremental matrix and do not by themselves
justify consolidation.

See [`docs/PROJECTS.md`](docs/PROJECTS.md) and issue #1 for the complete source,
Zed, git-submodule, GitHub, and Linear routing contract.
