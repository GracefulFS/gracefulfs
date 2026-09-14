# Release Process

## Create a Release Candidate

1. Prepare `release-X.Y.Z` with the release workflow. Set `Cargo.toml` to
   `X.Y.Z-rc.N`, run `cargo check` to update `Cargo.lock`, then run
   `pwsh -File scripts/check.ps1` and `cargo audit --file Cargo.lock`.
   Commit both version files, complete review, and push the release branch.
2. Tag its reviewed commit (example for `0.1.0-rc.1`):

   ```powershell
   git fetch origin
   git tag -a v0.1.0-rc.1 origin/release-0.1.0 -m "GracefulFS 0.1.0-rc.1"
   git push origin v0.1.0-rc.1
   ```

3. In **Actions > Release**, review the tag, commit, checks, ZIP, and `SHA256SUMS`.
   Select **Review deployments > prerelease > Approve and deploy** to publish,
   or **Reject** to stop.

All release checks run sequentially, including for documentation-only changes.
After approval, the verified ZIP is published without rebuilding. It contains
`gracefulfs.exe`, `LICENSE`, and usage instructions.

## Approval Settings

Configured under **Settings > Environments > prerelease**; the publishing job
must reference `environment: prerelease`.

| Setting | Value |
| --- | --- |
| Reviewers (one approval required) | `dldyou`, `sakwon0416`, `sj20-11`, `creampuffshu` |
| Self-review | Allowed |
| Administrator bypass | Disabled |
| Allowed refs | Tags matching `v*-rc.*`; no branches |

## Verify Downloads

Download the ZIP and `SHA256SUMS` into an empty folder, then run:

```powershell
$checksum, $archive = (Get-Content SHA256SUMS).Trim() -split '  ', 2
if ((Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash -ine $checksum) {
    throw 'Checksum mismatch'
}
```

## Failed Runs

- For transient errors, rerun failed jobs and approve publishing again. If the
  artifacts have expired (14 days), rerun all jobs and review the rebuilt files.
- Existing releases are never overwritten. After a publishing error, inspect
  the release: verify downloads if complete; create the next RC if incomplete.
- For code or workflow fixes, update both version files and create a new RC tag
  after review. Never move or reuse a published tag.
