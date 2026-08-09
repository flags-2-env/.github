# Governance

## Canonical organization links

- GitHub organization: https://github.com/flags-2-env
- Public organization defaults: https://github.com/flags-2-env/.github
- Canonical Linear project: https://linear.app/denman/project/githubcomflags-2-env-05db5133a267
- Fleet tracking issue: https://github.com/ORESoftware/k8s-cluster/issues/1222

Organization owners are accountable for repository creation, visibility, access, archival, public defaults, and cross-repository governance. Repository maintainers own implementation quality and releases within published contracts. Material architecture decisions must be documented in the owning repository and reflected in interfaces, tests, deployment ownership, and observability expectations.

The canonical product source is `flags-2-env/flags-2-env`. Through 2026-08-19
inclusive, `ORESoftware/flags-2-env` is a supported compatibility mirror whose
`main` and shared tags must resolve to the same Git objects. Canonical-first
publication, compatibility fast-forward, and verified parity are one release
operation; drift stops publication until a reviewed roll-forward restores it.

Resolve conflicts semantically with complete historical and cross-repository context. Automated agents must not execute destructive or history-rewriting operations.
