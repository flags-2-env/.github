# Contributing

**Contribute to
[flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env).**
That repository is canonical for new library work. This `.github` repository
only owns the organization profile and shared community-health files.

## Where to file what

| Kind of change | Repository |
| --- | --- |
| Parser behavior, a new flag type, a bug in the C core | [flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env) |
| A language client (`clients/<lang>/`) | [flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env) |
| A new consumer fixture, or a broken one | [flags-2-env-test](https://github.com/flags-2-env-test) |
| Org profile copy, community health files | this repo |

The original
[`ORESoftware/flags-2-env`](https://github.com/ORESoftware/flags-2-env)
repository remains supported through **2026-08-19, inclusive**. Existing
issues and pull requests there will be triaged during the window, but accepted
source changes must land as ordinary reviewed commits in the canonical
repository before the compatibility repository is fast-forwarded. Do not open
the same report or pull request in both places.

## Working on the library

```bash
git clone https://github.com/flags-2-env/flags-2-env
cd flags-2-env
make all      # builds build/libflags2env.{so,dylib}, build/libflags2env.a, build/flags2env
make test     # borrow check, README snippets, parity, hardening suites
```

Every language client has a Docker image that runs its own suite against a
freshly built native core — see `clients/<lang>/Dockerfile`. Run the one for the
client you touched before opening a pull request.

## Ground rules

- The C core is the single source of truth for parse semantics. A client may not
  reimplement parsing; it binds to the ABI and converts the returned JSON.
- A change to parse behavior is a change to every client's observable output.
  Say so in the pull request description, and expect the consumer fixtures in
  `flags-2-env-test` to need a matching update.
- Keep `.cli-flags.toml` examples in documentation runnable. `make readme-test`
  executes the snippets in `README.md`.
- New client? It needs a Dockerfile, a test that asserts the same canonical
  parse result the other clients assert, and a `publish.sh`.

## Reporting security issues

Do not open a public issue. See [SECURITY.md](SECURITY.md).

## Rust-first scripts and automation

**Use Rust for first-party scripts and automation; do not add or extend Python scripts.** This includes build/CI helpers, test infrastructure, code generation, database/schema migration tooling, deployment/release helpers, project administration, and ad-hoc repository maintenance. Prefer maintained Rust CLIs or `cargo xtask`; shell and YAML should be thin wrappers when the platform requires them.

**Existing Python automation is migration debt.** Record each existing script and its callers, then replace with Rust while retaining arguments, exit behavior, data formats, error paths, security checks, and supported platforms. Validate positive and negative regression scenarios and update workflow, hook and documentation entry points before removing an old script. An issue or documentation PR is *not* evidence of completed migration.

**Python application/product code may be approved as a narrow exception** when its owner documents the scope, purpose, tradeoffs, and why Rust is not appropriate. Associated application-specific tests may follow that exception. A build script, CI task, generator, migration runner, or one-off automation cannot be relabeled "application code" to evade the rule.

Document repeatable `cargo` commands; cover meaningful failures and regressions, and use `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and appropriate end-to-end checks. Report checks as passed only when actually executed on the changed commit. Do not delete functioning scripts until an equivalent Rust path is tested.

The canonical organization `.github` repository supplies **contributor guidance**; GitHub does not automatically inherit arbitrary `AGENTS.md`, policies, or workflows. Individual repositories should synchronize agent instructions and explicitly enable relevant checks to make the rule enforceable.
