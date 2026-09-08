# Repository Governance

## Protected Branches

The `main` branch is the protected default branch.

All changes to `main` should be submitted through a pull request.

The following rules apply:

- Direct changes to `main` are not allowed.
- Force pushes to `main` are not allowed.
- Deleting `main` is not allowed.
- Pull request conversations must be resolved before merging.
- Pull requests are merged using squash merge.
- The final pull request title becomes the squash commit title.

## Commit Convention

Commits should follow this format:

`<type>: <short description> (#<issue-number>)`

Example:

`docs: document repository governance rules (#8)`

## Pull Request Convention

Pull request titles should follow this format:

`<Short Description> (#<issue-number>)`

Example:

`Document repository governance rules (#8)`

Local commits do not need to reference an issue. The issue reference is required in the final pull request title.

## Reviews

Pull requests targeting `main` require at least one approving review.

The most recent reviewable push must be approved by someone other than the person who pushed it.

Pull requests modifying protected areas require approval from the designated code owners defined in `.github/CODEOWNERS` before merging.

## Signed Commits

All commits pushed to `main` must have a verified signature.

Contributors should configure commit signing before opening a pull request targeting `main`.

## Status Checks

Required status checks will be enabled after the CI workflows are added.

The required checks should include formatting, linting, build, and test validation when those workflows are available.

## Code Owners

Repository ownership is managed via `.github/CODEOWNERS`.

The following ownership maintenance rules apply:

- Changes to `.github/CODEOWNERS` must be reviewed and approved by a maintainer.
- When new directories, modules, or key paths (such as `.github`, documentation, source, tests, and workflows) are added or restructured, ownership must be updated accordingly.
- Ownership changes must never leave any protected path without an accountable reviewer.
- A default reviewer rule (`*`) must always be maintained to ensure complete coverage.

## Exceptions

Emergency bypasses should be limited to repository administrators and documented afterward in the relevant issue or pull request.
