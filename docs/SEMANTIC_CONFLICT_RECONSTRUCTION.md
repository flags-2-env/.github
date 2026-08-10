# Semantic conflict reconstruction policy

Flags-to-environment behavior is a public compatibility contract. A conflicted or stale branch must therefore be reconciled by intent and evidence, not by selecting one side of a textual conflict.

This policy applies across the `flags-2-env` organization and to upstream producer work in `ORESoftware/flags-2-env`.

## Non-negotiable rules

- Never force-push over source evidence supplied for reconciliation.
- Never resolve a non-trivial conflict with a blanket `ours`, `theirs`, or equivalent whole-file choice.
- Never merge a stale branch merely because its isolated tests pass.
- Never discard current-main packaging, release, security, fixture, or cross-platform fixes to recover an older feature.
- Never treat GitHub's synthetic pull-request merge ref as identical to the immutable source branch object.
- Never weaken a test so that incompatible behavior appears green.

## Required evidence

Record all of the following before changing code:

1. the current target branch and exact target SHA;
2. the source branch and exact 40-character source SHA;
3. the merge base and relevant commit history for both lines;
4. the changed files and independently valuable behavior introduced by each line;
5. source-head results, current-main results, and synthetic-merge results as separate facts;
6. fixture, packaging, release, and supported-platform changes that landed after the source branch diverged.

When repository CI cannot express those distinctions, add an external exact-head lane in `flags-2-env-test`. The lane must verify the checked-out SHA before running tests.

## Reconstruction workflow

### 1. Decompose by behavior

Turn the stale branch into independently reviewable feature slices. For the current `ORESoftware/flags-2-env#27` case, the slices are:

1. dotenv file ordering and secret-safe `doctor` diagnostics;
2. terminal context and `requires_tty` enforcement;
3. bundled Rust parser parity and fixture hardening.

Do not recreate an old branch as one monolithic commit merely because it began that way.

### 2. Rebuild on current `main`

Implement each slice from the current target head. Preserve current command scoping, unknown-flag policy, generated bindings, native-library names, packaging, release artifacts, and fixture repairs unless the reviewed behavior intentionally changes them.

The typed application parser remains authoritative. `flags-2-env` normalizes argv into environment-backed configuration and audits that boundary; reconstruction must not silently move application validation into a second conflicting parser.

### 3. Certify immutable heads

For every reconstructed slice:

- use a full commit SHA, never a mutable branch name, in external certification;
- verify `git rev-parse HEAD` equals the expected SHA;
- run the producer's own suite;
- run the independent feature matrix;
- cover Linux and macOS, plus Windows wherever the changed surface is platform-sensitive;
- keep source-head failures visible instead of rewriting the historical branch to make evidence green.

A green historical source head is behavioral evidence, not merge approval. A failing historical source head can still identify useful behavior, but that behavior must pass on the reconstructed current-main head.

### 4. Report integration honestly

PR descriptions must state:

- exact base and head SHAs;
- recovered feature slice and explicit non-goals;
- source-head, current-main, and merge-integration results;
- semantic decisions made where histories disagreed;
- whether the PR is evidence-only, reconstruction-ready, or production-ready;
- any remaining consumer or platform certification.

## Current reference lanes

- `flags-2-env-test/feature-matrix#10` pins the conflicted PR #27 head as immutable behavioral evidence and runs producer plus external matrices.
- `ORESoftware/flags-2-env#44` is an independent current-main repair for the cross-platform Python native-library defect exposed by that lane.

PR #44 does not absorb the dotenv, terminal, or bundled-parser feature slices. Those remain separate reconstruction work by design.

## Merge readiness

A reconstructed PR may leave draft only when:

- its exact head is mergeable against the current target;
- all applicable producer and external checks are green;
- platform-sensitive behavior has the required matrix evidence;
- the PR describes the semantic merge decisions and omitted slices;
- no source history was overwritten.

Evidence-only lanes should remain draft even when they are useful. Runner-blocked or account-blocked lanes must not be represented as tested merely because GitHub created a workflow run object.
