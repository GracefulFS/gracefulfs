# Contributing to GracefulFS

Thank you for your interest in contributing to GracefulFS.

## Before You Start

Please searching issues before opening a new one.

Use the repository's issue templates when reporting:

- Bug report
- Feature request
- Task

For larger changes, open of discuss an issue before starting implementation.

## Windows Development Prerequisites

GracefulFS targets read-only analysis of Windows local storage.
The MVP does not modify, delete, or move user files.

### C++ Build Tools and Windows SDK

Install Visual Studio 2022 or Build Tools for Visual Studio 2022.
Select the **Desktop development with C++** workload and ensure the following components are installed:

- MSVC v143 C++ x64/x86 build tools
- Windows 11 SDK (for example, 10.0.26100.0)

An existing Visual Studio installation with these components is sufficient.
The Visual Studio IDE is optional.

The maintainer's current environment includes MSVC 14.38.33130 and Windows SDK versions 10.0.26100.0.
These are reference versions, not enforced minimum requirements.

### Rust Toolchain

Install Rust using Rustup and use the MSVC toolchain.
Run commands from the repository root so Rustup uses `rust-toolchain.toml`, which specifies:

- Rust: 1.98.1
- Target: x86_64-pc-windows-msvc
- Components: rustfmt, clippy

### Verify the Setup

```bash
rustup show     # 1.98.1-x86_64-pc-windows-msvc
rustc --version # rustc 1.98.1
cargo --version # cargo 1.98.1
rustup target list --installed
rustup component list --installed

cargo metadata --no-deps
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --locked --target x86_64-pc-windows-msvc
cargo test
```

### Troubleshooting

- **`cargo` is not recognized:** Restart the terminal after installing Rustup and check that `%USERPROFILE%\.cargo\bin` is on PATH.
- **`link.exe` is missing or Windows SDK libraries cannot be found:** Open Visual Studio Installer and add the C++ build tools and Windows SDK components listed above.
- **The target or Rust components are missing:** Run the following commands from the repository root:

```bash
rustup target add x86_64-pc-windows-msvc
rustup component add rustfmt clippy
```

## Project Layout and Commands

The repository uses a single root Cargo package. `src/main.rs` is the
application entry point, and `tests/` is reserved for integration tests.
The initial application exits without performing filesystem operations.

Run the following commands from the repository root on Windows using the
toolchain pinned in `rust-toolchain.toml`:

```bash
cargo metadata --no-deps
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --locked --target x86_64-pc-windows-msvc
```

To run formatting checks, linting, building, and tests together in PowerShell:

```powershell
.\scripts\check.ps1
```

The script runs from the repository root, uses the pinned Rust toolchain and
the single target in `rust-toolchain.toml`, and stops at the first failed check
with a non-zero exit code. It does not apply formatting changes. Install the
required toolchain and build tools before running it.

## Continuous Integration

The `CI` workflow runs on pull requests targeting `main`, `develop`, or
`release-*`, and on pushes to those branches. One Windows x64 MSVC job runs
Rust setup, formatting, Clippy, and a debug build sequentially. A failed step
prevents later checks from running. The Rust toolchain and build target come
from `rust-toolchain.toml`. The runner provides MSVC build tools and the Windows
SDK. Build failures fail the CI check, and `--locked` prevents Cargo from
updating `Cargo.lock`.

To reproduce the Rust checks locally, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --locked --target x86_64-pc-windows-msvc
```

When the full PR diff or push comparison contains only Markdown (`.md`) files,
the workflow reports a successful skip without setting up Rust or running its
checks. Deleted files and both sides of renames are included. Mixed changes,
empty comparisons, and uncertain change detection run the Rust checks.
The workflow itself is not skipped through path filters.

Automated tests will be added in issue #18.

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
