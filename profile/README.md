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
