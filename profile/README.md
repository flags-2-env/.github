# flags-2-env

> **The canonical source code lives at
> [github.com/flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env).**
> The original
> [github.com/ORESoftware/flags-2-env](https://github.com/ORESoftware/flags-2-env)
> remains a supported compatibility mirror through **2026-08-19, inclusive**.

`flags-2-env` is a C library — with bindings for ~35 languages — that parses CLI
flags declared in a project-local `.cli-flags.toml` into a string-to-string map
of environment variable overrides. Declare your flags once; every runtime in
your stack reads the same contract.

## Where to go

| I want to… | Go to |
| --- | --- |
| Read or clone the source | [flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env) |
| File an issue or a PR | [flags-2-env/flags-2-env/issues](https://github.com/flags-2-env/flags-2-env/issues) |
| Report a vulnerability privately | [Canonical security advisory](https://github.com/flags-2-env/flags-2-env/security/advisories/new) |
| See it consumed from 12 languages | [flags-2-env-test](https://github.com/flags-2-env-test) |
| Install with Zed | `flags-2-env/flags-2-env@^0.3` |
| Install the Node package | `npm install @oresoftware/f2e` |

## Ten-day compatibility window

New Git references and Zed manifests should use the canonical coordinates:

```text
https://github.com/flags-2-env/flags-2-env
flags-2-env/flags-2-env@0.3.0
```

Through 2026-08-19 inclusive, these original coordinates remain supported:

```text
https://github.com/ORESoftware/flags-2-env
oresoftware/flags-2-env@0.3.0
```

Both Git repositories must resolve `main` and every shared tag to identical
objects. The Zed packages have distinct identities because the registry cannot
alias coordinates, but both are published from the same immutable tagged
source. Publication stops on drift; canonical changes are reviewed first and
the compatibility repository is fast-forwarded second.

## What the contract looks like

```toml
# .cli-flags.toml
[flags.port]
env = "PORT"
aliases = ["port", "listen-port"]
short = "p"
type = "integer"
default = 3000
help = "TCP port for the app listener."

[flags.debug]
env = "DEBUG"
aliases = ["debug"]
short = "d"
type = "bool"
default = "false"
true_aliases = ["t", "1", "yes"]
false_aliases = ["f", "0", "no"]
help = "Enable debug logging."
```

```console
$ flags2env --port 8181 --debug=t
{"PORT":"8181","DEBUG":"true"}
```

The native core is C. Each runtime client binds to the same small ABI and turns
the returned JSON object into that language's native map type, so the parse
result is identical whether you call it from Rust, Ruby, or Gleam.

## Memory safety

The C core is gated by layered, self-verifying checks in the canonical
repository — every PR must pass:

- **A custom borrow checker**
  ([`tools/borrow-checker/`](https://github.com/flags-2-env/flags-2-env/tree/main/tools/borrow-checker))
  — a flow-sensitive ownership analysis of the library's own
  `F2E_OWNED_RESULT` / `F2E_TAKES_OWNED_ARG_1` contract. It rejects
  double-frees, use-after-free, unchecked-allocation dereferences, leaks,
  dangling stack returns, and undeclared ownership transfers across the
  public ABI, and it infers contracts for file-local helpers so internals
  need no annotation.
- **Formal proofs**
  ([`formal/`](https://github.com/flags-2-env/flags-2-env/tree/main/formal))
  — CBMC model-checks bounded calls into the real parser and the
  terminal-context scanners; Z3 proves parser dispatch invariants **and the
  borrow checker's own state machine** (no bounded trace can reach a
  use-after-free or double-free unflagged); Kani proves the mirrored Rust
  dispatch rules.
- **Sanitizers and fuzzing** — ASan/UBSan across compilers, an allocation-
  failure harness, and a parser fuzzer, plus the clang static analyzer as an
  independent second opinion.

The consumer fixtures in
[flags-2-env-test](https://github.com/flags-2-env-test) re-run the vendored
borrow checker against the exact sources each language binding compiles, so
a submodule pin can never advance to a revision the checker rejects.

## Organizations

- **flags-2-env** (this org) — canonical product source, releases, issues, org
  profile, and community-health files.
- **[ORESoftware](https://github.com/ORESoftware)** — original source location
  and supported compatibility mirror through 2026-08-19 inclusive.
- **[flags-2-env-test](https://github.com/flags-2-env-test)** — one consumer
  repository per language plus interoperability and conformance suites. These
  repositories prove the contract end-to-end; they are not source publishers.

## License

`flags-2-env` is MIT licensed. See
[the license in the canonical repository](https://github.com/flags-2-env/flags-2-env/blob/main/LICENSE).


<!-- ore-org-baseline:begin -->
## Planning and governance

- Canonical Linear project: https://linear.app/denman/project/githubcomflags-2-env-05db5133a267
- Organization defaults: https://github.com/flags-2-env/.github
- Canonical agent policy: https://github.com/flags-2-env/.github/blob/main/agents.md
- Security policy: https://github.com/flags-2-env/.github/security/policy

Repositories in this organization use semantic conflict resolution with 3–10 relevant prior commits when useful, full cross-repository context, pull-request delivery, and a hard automated-agent denylist for destructive or history-rewriting operations.
<!-- ore-org-baseline:end -->
