# flags-2-env

> **The source code lives at [github.com/ORESoftware/flags-2-env](https://github.com/ORESoftware/flags-2-env).**
> This organization does not host the implementation. It holds the name, the
> org-level profile, and shared community-health files.

`flags-2-env` is a C library — with bindings for ~35 languages — that parses CLI
flags declared in a project-local `.cli-flags.toml` into a string-to-string map
of environment variable overrides. Declare your flags once; every runtime in
your stack reads the same contract.

## Where to go

| I want to… | Go to |
| --- | --- |
| Read or clone the source | [ORESoftware/flags-2-env](https://github.com/ORESoftware/flags-2-env) |
| File an issue or a PR | [ORESoftware/flags-2-env/issues](https://github.com/ORESoftware/flags-2-env/issues) |
| See it consumed from 12 languages | [flags-2-env-test](https://github.com/flags-2-env-test) |
| Install the Node package | `npm install @oresoftware/f2e` |

**Nothing here is a fork or a mirror.** If a repository in this organization
ever appears to contain library source, treat it as stale and prefer
`ORESoftware/flags-2-env`, which is canonical.

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
  ([`tools/borrow-checker/`](https://github.com/ORESoftware/flags-2-env/tree/main/tools/borrow-checker))
  — a flow-sensitive ownership analysis of the library's own
  `F2E_OWNED_RESULT` / `F2E_TAKES_OWNED_ARG_1` contract. It rejects
  double-frees, use-after-free, unchecked-allocation dereferences, leaks,
  dangling stack returns, and undeclared ownership transfers across the
  public ABI, and it infers contracts for file-local helpers so internals
  need no annotation.
- **Formal proofs**
  ([`formal/`](https://github.com/ORESoftware/flags-2-env/tree/main/formal))
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

- **[ORESoftware](https://github.com/ORESoftware)** — canonical source of
  `flags-2-env` and the rest of the ORE Software fleet.
- **flags-2-env** (this org) — name reservation, org profile, community health
  files. No implementation code.
- **[flags-2-env-test](https://github.com/flags-2-env-test)** — one consumer
  repository per language, each vendoring `ORESoftware/flags-2-env` and proving
  the contract end-to-end in Docker.

## License

`flags-2-env` is MIT licensed. See
[the license in the canonical repository](https://github.com/ORESoftware/flags-2-env/blob/main/LICENSE).
