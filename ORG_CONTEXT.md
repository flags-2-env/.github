# Organization context

Three GitHub organizations participate in the `flags-2-env` project. The
product source has one canonical repository, one time-bounded compatibility
mirror, and a separate consumer-test organization.

## `flags-2-env` — canonical product organization

<https://github.com/flags-2-env/flags-2-env>

The C core (`src/parser.c`), per-language clients under `clients/`, CLI, test
suite, release automation, and issue tracker live here. Clone this repository.
File new issues here. Open new pull requests here.

Package identities published from it:

| Registry | Identity |
| --- | --- |
| npm | `@oresoftware/f2e` |
| crates.io | `flags2env` |
| PyPI | `flags2env` |
| Maven Central | `com.oresoftware:flags2env` |
| pub.dev | `flags2env` |
| Hex | `flags2env` |
| zed-pkg | `flags-2-env/flags-2-env@0.3.0` |

Organization-wide profile and community-health files live in
[`flags-2-env/.github`](https://github.com/flags-2-env/.github), but the
implementation belongs in the sibling `flags-2-env/flags-2-env` repository.

## `ORESoftware` — compatibility source

<https://github.com/ORESoftware/flags-2-env>

The original repository remains a supported, commit-identical compatibility
mirror through **2026-08-19, inclusive**. It keeps its existing issue and
pull-request history, and immutable references to it remain valid during the
window. New source references, issues, pull requests, releases, and security
reports use the canonical repository.

During the window, canonical `main` and every shared tag must have the same
object ID in both repositories. Publish canonical first, fast-forward the
compatibility repository second, and stop publication if they diverge. Repair
drift with reviewed roll-forward work on the canonical repository followed by
a compatibility fast-forward; never manufacture parity by rewriting history.

Zed package identities cannot be redirected or aliased, so the transition also
publishes `oresoftware/flags-2-env@0.3.0` from the same immutable tagged source
as canonical `flags-2-env/flags-2-env@0.3.0`. Their embedded identities and
therefore artifact digests differ; their remaining source files must not.

After 2026-08-19, the compatibility repository may remain as a read-only
historical source and redirect. Compatibility support must not be removed
before the inclusive cutoff.

## `flags-2-env-test` — consumer fixtures

<https://github.com/flags-2-env-test>

The organization contains one small consumer application per supported
language, a cross-language feature matrix, SOPS/Just/Nix interoperability
fixtures, and dedicated contract, recovery, upgrade, and security suites.
Language fixtures declare flags in `.cli-flags.toml` and run the exact pinned
library source in Docker to prove parse parity across runtimes.

These repositories are fixtures, not products. They deliberately pin the
upstream commit they verified and are allowed to break loudly when the contract
changes.

## Verified fleet state — 2026-08-09

The 12 language fixtures and `feature-matrix` still declare the compatibility
`oresoftware/flags-2-env = "^0.2.0"` coordinate and materialize under
`.vendor/.zed`. Their gitlinks use
`.vendor/.zed/oresoftware/flags-2-env`, making the committed checkout an
intentional offline stand-in rather than an accidental second source tree.
Those immutable pins remain supported during the transition window.

When a fixture migrates, change the dependency to
`flags-2-env/flags-2-env = "^0.3"` and move the gitlink to
`.vendor/.zed/flags-2-env/flags-2-env` in the same reviewed change. Do not
materialize both coordinates. The common workflow must verify the
manifest/path relationship, confirm the immutable pin belongs to the canonical
upstream, run the Docker contract, and deliberately mutate the flag contract
to prove the assertions can fail. Resolver-generated `.zpkg.lock` provenance
must never be hand-authored.

The `sops-just`, `sops-nix`, `sops-just-nix`, and `sops-just-nix-zpkg`
repositories are now populated interoperability fixtures. Four additional
repositories exercise contract conformance, chaos recovery, upgrade
compatibility, and security boundaries. These repositories consume or test the
product contract; none is an alternate source publisher.

See [`docs/PROJECTS.md`](docs/PROJECTS.md) and issue #1 for the source, Zed,
git-submodule, GitHub, and Linear routing history.
