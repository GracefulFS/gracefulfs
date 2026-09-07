# Contributing to GracefulFS

Thank you for your interest in contributing to GracefulFS.

## Before You Start

Please searching issues before opening a new one.

Use the repository's issue templates when reporting:

- Bug report
- Feature request
- Task

For larger changes, open of discuss an issue before starting implementation.

## Rust Configuration

```bash
rustup show     # 1.98.1-x86_64-pc-windows-msvc
rustc --version # rustc 1.98.1
cargo --version # cargo 1.98.1
```

## Issues

A good issue should:

- Describe on problem, proposal, or decision.
- Explain the expected outcome.
- Include relevant context and reproduction steps when applicable.
- Avoid including secrets or personal information.

Maintainers will review new issues, apply appropriate labels and milestones, and identify follow-up work when necessary.

## Labels and Triage

New issues start with the `status: needs-triage` label.

Maintainers review each issue and assign the appropriate labels before
moving it to `Ready` in the GitHub Project.

### Label Categories

- `type:*`: The kind of work. Assign one.
- `phase:*`: The project phase. Assign one.
- `area:*`: The affected part of the project. Multiple labels are allowed.
- `priority:*`: The urgency of the work. Assign one.
- `risk:*`: Relevant compatibility, security, data, or performance risks.
- `status:*`: Temporary triage status.
- `duplicate`, `invalid`, and `wontfix`: Terminal issue outcomes.

### Triage Statuses

- `status: needs-triage`: The issue has not been fully reviewed.
- `status: needs-info`: Additional information or a decision is required.
- `status: blocked`: Work cannot proceed because of a dependency or external blocker.

These status labels are mutually exclusive. Remove `status: needs-triage`
after triage is complete. Remove `status: needs-info` or `status: blocked`
when the issue is ready to proceed.

### Project Workflow

The GitHub Project tracks work progress:

`Backlog → Ready → In progress → In review → Done`

- `Backlog`: New or untriaged issues.
- `Ready`: Issues that have completed triage and are ready to start.
- `In progress`: Work is actively being implemented.
- `In review`: A pull request is open and awaiting review.
- `Done`: The pull request has been merged and the issue is complete.

Labels describe issue classification, while Project columns describe
the current stage of work. Do not create progress labels such as
`status: in-progress` or `status: done`.

## Branches

Create a branch from `main` for each issue.

Use the following naming convention:

`<issue-number>-<short-description>`

Example:

`1-define-gracefulfs-product-scope-and-mvp-boundaries`

Do not mix unrelated changes in the same branch.

## Pull Requests

Every pull request should:

- Reference the related issue.
- Explain what changed and why.
- Describe any validation performed.
- Update relevant documentation.
- Keep the change focused and reviewable.
- Complete the pull request template.

For documentation-only changes, state that no code tests were required.

## Code Ownership

GracefulFS uses a path-based ownership model to ensure changes to critical areas are reviewed by the appropriate maintainers.

- **CODEOWNERS Mapping:** Path-specific review rules are defined in [.github/CODEOWNERS](.github/CODEOWNERS). Key paths—including `.github/`, workflows, source code, tests, and documentation—are mapped to specific maintainers or default reviewers.
- **Required Reviews:** Pull Requests modifying protected paths will automatically request reviews from assigned Code Owners. These reviews must be approved before merging, as enforced by repository rulesets and branch protection policies.
- **Maintaining Ownership:** As repository areas evolve or new modules are added, ownership rules must be updated accordingly. Any change to `.github/CODEOWNERS` or protected paths must be reviewed and approved by an existing maintainer to prevent leaving paths without an accountable reviewer.

## Review

Maintainers review changes for:

- Alignment with the related issue.
- Correctness and clarity.
- Security and privacy concerns.
- Unnecessary scope expansion.
- Documentation consistency.

Contributors should respond to review feedback and resolve outstanding discussions before merge.

See [Repository Governance](REPOSITORY_GOVERNANCE.md) for branch protection, review, and merge policies.

## Communication

Please communicate respectfully and constructively.

Keep technical criticism focused on the work, not on individuals. Do not use issues or pull requests for harassment, spam, or personal disputes.

## Related Policies

- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [Repository Ownership](.github/CODEOWNERS)
- [Branch and Ruleset Policy](https://github.com/dldyou/gracefulfs/issues/8)
