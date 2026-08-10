# Branching and deployment

## Canonical organization links

- GitHub organization: https://github.com/flags-2-env
- Public organization defaults: https://github.com/flags-2-env/.github
- Canonical Linear project: https://linear.app/denman/project/githubcomflags-2-env-05db5133a267
- Fleet tracking issue: https://github.com/ORESoftware/k8s-cluster/issues/1222

Use additive feature or fix branches and pull requests. Preserve the repository's configured integration and release model, required reviews, branch protection, rulesets, security gates, and environment approvals. Deploy immutable, reviewed artifacts through the owning infrastructure and GitOps repositories; do not treat a source checkout as a production deployment mechanism.

Never force-push, rewrite shared history, bypass checks, or destroy state to advance a deployment. Prefer reversible roll-forward changes and document artifact identity, migration effects, observability, and recovery steps.
During the compatibility window ending 2026-08-19 inclusive, release from
`flags-2-env/flags-2-env` first and fast-forward
`ORESoftware/flags-2-env` second. A release is incomplete until `main`, shared
tags, and release inputs have verified parity. If the repositories diverge,
stop publication and restore parity with reviewed roll-forward commits; never
create a compatibility-only merge commit or rewrite a public ref.
