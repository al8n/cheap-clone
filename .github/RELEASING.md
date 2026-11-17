# Releasing 1.0.0

This release publishes `cheap-clone-derive` 1.0.0, then `cheap-clone` 1.0.0,
from the same commit and the `v1.0.0` tag. The runtime's optional macro
dependency must also select version 1.0.0.

## Trusted Publishing setup

Both crates must already exist on crates.io. The published derive 0.1.0
satisfies that requirement; it is not republished by this workflow.

Configure a GitHub Trusted Publisher in **each** crate's crates.io settings:

| Field | Value |
| --- | --- |
| Repository owner | `al8n` |
| Repository name | `cheap-clone` |
| Workflow filename | `release.yml` |
| Environment name | `crates-io` |

- [cheap-clone settings](https://crates.io/crates/cheap-clone/settings)
- [cheap-clone-derive settings](https://crates.io/crates/cheap-clone-derive/settings)
- [GitHub environments](https://github.com/al8n/cheap-clone/settings/environments)

The publish job uses the `crates-io` GitHub environment and exchanges its OIDC
identity for a temporary crates.io token. No repository token secret is needed.
The authentication action revokes the temporary token at the end of the job.

## Validate the release PR

Run CI on the PR. The release workflow can also be dispatched on its branch:

```sh
gh workflow run release.yml --repo al8n/cheap-clone --ref release/1.0.0 -f tag=v1.0.0
```

A manual dispatch only validates versions, semver compatibility, dependency
advisories, workspace tests, packaging, and the publish dry run. It does not
publish packages or create a GitHub release.

## Publish after merging

Wait for CI to pass on the exact merge commit on `main`, then run:

```sh
git switch main
git pull --ff-only origin main
git tag -a v1.0.0 -m "Release cheap-clone and cheap-clone-derive 1.0.0"
git push origin v1.0.0
```

The tag must point to the current `origin/main` commit. The workflow validates
both crate versions and requires a successful `main` CI run for that commit.
It dry-runs the entire workspace before requesting an OIDC token, publishes
the derive crate first, then publishes the runtime. Cargo waits for a published
package to appear in the registry index before returning.

After both packages are published, the workflow creates the GitHub release
using `RELEASE.md`. A rerun skips a package only when crates.io confirms its
version, GitHub publisher repository, commit, and non-yanked state match this
run. If publication succeeds but Cargo times out waiting for the index, verify
the registry state before rerunning the failed jobs; do not move the tag.
