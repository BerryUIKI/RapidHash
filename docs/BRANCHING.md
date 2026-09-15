# Branching and Pull Request Policy

## Protected Branches: `main` and `dev`

RapidHash utilizes a two-tier protected branch architecture:

1. **`main` (Production / Stable)**:
   - Contains only verified, production-ready, releasable code.
   - Direct commits and direct pushes are strictly prohibited.
   - Changes reach `main` **only** through Pull Requests from the `dev` branch after milestones or releases are stabilized and verified.

2. **`dev` (Active Integration)**:
   - The primary integration branch where day-to-day development occurs.
   - Direct commits and direct pushes are strictly prohibited.
   - All topic, feature, fix, and documentation branches branch off `dev` and target `dev` via Pull Requests.
   - All commits and Pull Requests targeting `dev` must be written entirely in English.

Both `main` and `dev` enforce:
- Mandatory Pull Requests before merge.
- Linear history (squash or rebase merge only; merge commits disabled).
- Force pushes, branch deletions, and administrative bypass are prohibited.
- Resolved review conversations before merge.

Release tags are immutable. A temporary `release/<version>` branch may be used
only when stabilizing a release that cannot be stabilized on `dev`.

## Branch Names

Use lowercase ASCII, hyphens, and this structure:

```text
<type>/<issue-or-noissue>-<short-description>
```

Allowed types:

- `feature` for user-visible behavior;
- `fix` for defects;
- `docs` for documentation only;
- `refactor` for behavior-preserving changes;
- `perf` for measured performance work;
- `test` for test-only work;
- `build` for build or packaging;
- `ci` for automation;
- `chore` for maintenance;
- `release` for release stabilization; and
- `hotfix` for urgent supported-release fixes.

Examples:

```text
docs/noissue-project-foundation
feature/42-streaming-sha256
fix/87-manifest-path-escape
```

Delete ordinary branches after merge.

## Small Pull Requests

Every pull request must deliver one reviewable outcome. Prefer under 300 changed
lines excluding generated files, fixtures, lockfiles, and unavoidable license
text. A larger change must explain why it cannot be split safely.

Split work by dependency order. A typical feature uses separate pull requests
for contract/types, core behavior, adapter integration, and user interface when
those pieces can be reviewed independently without leaving `main` broken.

Do not mix refactoring, dependency upgrades, formatting sweeps, and new behavior
unless they are inseparable.

## Frequent, Coherent Commits

Commit whenever a small, independently understandable step is complete and
validated. Each commit must build or preserve the documented bootstrap state,
have a Conventional Commit message, and avoid unrelated changes.

Good commit boundaries include:

- add a policy or public contract;
- add failing tests for one behavior;
- implement the behavior;
- connect one adapter;
- update the relevant documentation; and
- address one review concern.

Frequent does not mean noisy: do not commit broken experiments, mechanical
fixups, or single-character corrections that belong with the preceding change.
Before merge, maintainers may request autosquash of fixup commits while retaining
meaningful implementation history. The repository's default merge strategy is
squash merge unless preserving multiple independently valuable commits is useful.

## Pull Request Stack

Dependent changes may use stacked pull requests. Each PR identifies its base and
dependency, remains individually reviewable, and is rebased after its parent
merges. Do not combine a stack merely to reduce PR count.

## Required Checks and Reviews

Branch protection should require:

- a pull request before merging;
- all configured status checks;
- the branch to be current before merge when supported;
- resolved review conversations;
- at least one approving review once a second maintainer is available;
- linear history;
- no force pushes; and
- no branch deletion.

During single-maintainer bootstrap, PRs remain mandatory but the approval count
may be zero because GitHub does not allow authors to approve their own changes.
Raise the requirement to one as soon as another trusted maintainer is active.
Administrative bypass is reserved for repository recovery and documented
security emergencies.

## Merge and Promotion Policy

Use squash merge by default with a Conventional Commit title in English. Rebase merge
may be used when every commit is deliberately preserved. Merge commits are disabled
on both `main` and `dev` to retain linear history.

All changes must first be merged into `dev` via a Pull Request. Once a milestone
or release cycle on `dev` is validated, a promotion Pull Request is opened from `dev`
to `main`. Fixes for supported releases may be backported to a dedicated release
branch. Never develop new features directly on `main` or release branches.
