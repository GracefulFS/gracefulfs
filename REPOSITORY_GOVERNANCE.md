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

Code Owner review is not currently required because the repository ownership model and `CODEOWNERS` file have not yet been established.

See Issue #9.

## Signed Commits

All commits pushed to `main` must have a verified signature.

Contributors should configure commit signing before opening a pull request targeting `main`.

## Status Checks

Required status checks will be enabled after the CI workflows are added.

The required checks should include formatting, linting, build, and test validation when those workflows are available.

## Code Owners

Code owner review will be enabled after the repository ownership model and `CODEOWNERS` file are established.

See Issue #9.

## Exceptions

Emergency bypasses should be limited to repository administrators and documented afterward in the relevant issue or pull request.
