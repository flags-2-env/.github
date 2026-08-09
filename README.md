# flags-2-env/.github

Organization-wide community health files for the `flags-2-env` GitHub
organization.

The canonical library source is
**[flags-2-env/flags-2-env](https://github.com/flags-2-env/flags-2-env)**.
Use its [issue tracker](https://github.com/flags-2-env/flags-2-env/issues),
[pull requests](https://github.com/flags-2-env/flags-2-env/pulls),
[releases](https://github.com/flags-2-env/flags-2-env/releases), and
[private security advisories](https://github.com/flags-2-env/flags-2-env/security/advisories/new)
for new library work.

The original
[`ORESoftware/flags-2-env`](https://github.com/ORESoftware/flags-2-env)
repository is a supported compatibility mirror through **2026-08-19,
inclusive**. Existing immutable references and existing issue or pull-request
history there remain supported during the transition, but new references and
new work should use the canonical repository.

| File | Applies to |
| --- | --- |
| [`profile/README.md`](profile/README.md) | The public org landing page at <https://github.com/flags-2-env> |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Every repo in this org that lacks its own |
| [`SECURITY.md`](SECURITY.md) | Vulnerability reporting for the project |
| [`SUPPORT.md`](SUPPORT.md) | Where to ask questions |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Conduct across the org |
| [`ORG_CONTEXT.md`](ORG_CONTEXT.md) | How this org relates to `ORESoftware` and `flags-2-env-test` |

## Why the org exists

This organization owns the product source, releases, public project identity,
and shared community-health policy. The paired
[`flags-2-env-test`](https://github.com/flags-2-env-test) organization owns
consumer fixtures and conformance suites; it does not replace or fork the
library source.

## Source and package transition

| Surface | Canonical | Compatibility through 2026-08-19 inclusive |
| --- | --- | --- |
| GitHub source | [`flags-2-env/flags-2-env`](https://github.com/flags-2-env/flags-2-env) | [`ORESoftware/flags-2-env`](https://github.com/ORESoftware/flags-2-env) |
| zed-pkg | `flags-2-env/flags-2-env@0.3.0` | `oresoftware/flags-2-env@0.3.0` |

The Git repositories must resolve `main` and every shared tag to identical Git
objects during the support window. Releases land on the canonical repository
first and are then fast-forwarded to the compatibility repository; any drift is
a release blocker. The two Zed coordinates are separate publications because
Zed has no package-alias field, but both must be built from the same immutable
tagged source. See the canonical
[source-migration contract](https://github.com/flags-2-env/flags-2-env/blob/main/docs/source-migration.md)
for the verification and cutoff procedure.


<!-- ore-org-baseline:begin -->
## Organization-wide defaults

This public repository is the canonical source for GitHub-supported community-health fallbacks, organization profile content, contribution guidance, public security/support policy, issue and pull-request templates, and agent-governance declarations for [`flags-2-env`](https://github.com/flags-2-env).

## Canonical organization links

- GitHub organization: https://github.com/flags-2-env
- Public organization defaults: https://github.com/flags-2-env/.github
- Canonical Linear project: https://linear.app/denman/project/githubcomflags-2-env-05db5133a267
- Fleet tracking issue: https://github.com/ORESoftware/k8s-cluster/issues/1222

## Safety baseline

All Git conflicts must be resolved semantically with full historical, repository-wide, organization-wide, and relevant external-organization context. Automated agents are hard-denied from destructive or history-rewriting operations, including all forms of `git stash`, `git reset`, `git clean`, `git filter-repo`, force pushing, destructive deletion, data or infrastructure teardown, credential revocation, and policy bypass.

## GitHub inheritance boundary

GitHub can use supported community-health files from a public organization `.github` repository as fallbacks and can render `profile/README.md` on the organization page. `agents.md`, `AGENTS.md`, Copilot instructions, workflows, settings, rulesets, branch protections, permissions, and secrets are not automatically inherited merely because they exist here. Each repository must carry or synchronize compatible local policy and explicitly call reusable workflows where enforcement is required.

Generated managed-policy version: `2026-08-08`.
<!-- ore-org-baseline:end -->
