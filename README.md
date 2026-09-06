# What is GracefulFS?

[logo after here]

Graceful File System Management, GracefulFS is a Window filesystem analysis tool.

## Who is it for?

GracefulFS is for Windows developers and power users who need to find large files, understand folder usage, and inspect disk consumption safely.

## Contributing

Please read the following documents before contributing:

- [Contributing Guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [Repository Governance](REPOSITORY_GOVERNANCE.md)

## MVP Workflow

1. Select a local Windows drive or folder.
2. Scan its contents without modifying any files.
3. Collect file metadata and size information.
4. Show folder usage and the largest files.
5. Allow the user to monitor progress, cancel the scan, and exclude paths.

## MVP Scope

Included:

- Windows local drives and folders
- Read-only recursive scanning
- File size and basic metadata collection
- Folder usage summaries
- Large-file analysis
- Scan progress and cancellation
- Excluded paths

Excluded from the MVP:

- Automatic deletion or file movement
- Quarantine and recovery
- Duplicate-file detection
- Stale-file and history analysis
- Background monitoring and scheduled scans
- Network, cloud, and Docker storage analysis
- macOS and Linux support
- Desktop UI

## Safety Principle

The MVP never deletes, moves, or modifies user files. Any future cleanup action must require an explicit user-approved plan.

## Initial Platform

The initial target is Windows local storage. Exact Rust toolchain, Windows targets, and compatibility requirements are defined during the Rust foundation work.

## Product Direction

Future versions may add indexing, duplicate and stale-file analysis, cleanup recommendations, safe file actions, recovery, history, monitoring, and a Windows desktop application.

## Roadmap

See the [GitHub milestones](https://github.com/dldyou/gracefulfs/milestones).
