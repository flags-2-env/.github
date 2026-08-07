# Security policy

## Reporting a vulnerability

Report vulnerabilities through the **canonical repository**, not through this
organization:

<https://github.com/ORESoftware/flags-2-env/security/advisories/new>

If private advisory reporting is unavailable to you, email the maintainer listed
on <https://github.com/ORESoftware> instead. Do not open a public issue for a
vulnerability.

Please include: the affected client or the C core, the `.cli-flags.toml` and
argv that reproduce the issue, the observed result, and the result you expected.

## Scope

`flags-2-env` parses untrusted-ish input — command line arguments — against a
project-controlled TOML contract, in C, and hands the result to a dozen runtimes
through FFI. Memory-safety findings in `src/parser.c` and in the per-client
native shims are the highest-severity class and are treated as such.

In scope:

- Memory safety in the C core or in a client's native shim (out-of-bounds read
  or write, use-after-free, double free, unchecked allocation, integer overflow
  reaching an allocation size).
- A parse result that violates the declared contract in a way that grants
  privilege — for example an unknown flag being silently accepted into the
  environment map when strict parsing was requested.
- Leakage of an unrecognized flag's *value* into an error message or log, which
  can spill a mistyped credential.

Out of scope:

- A project author declaring a footgun in their own `.cli-flags.toml`.
- Denial of service from an argv the operator supplied to their own process.
- Findings in the fixtures under [flags-2-env-test](https://github.com/flags-2-env-test),
  which are throwaway test harnesses with no production role.

## Supported versions

The most recent release on the `main` branch of `ORESoftware/flags-2-env` is
supported. There are no long-term support branches.
