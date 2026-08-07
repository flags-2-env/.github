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
