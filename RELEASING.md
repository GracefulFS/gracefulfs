# Release Process

## Prerelease Environment

Manage approval settings under **Settings > Environments > prerelease**.
Publishing jobs must reference `environment: prerelease` to use these rules.

| Setting | Value |
| --- | --- |
| Required reviewers | `dldyou`, `sakwon0416`, `sj20-11`, `creampuffshu` |
| Required approvals | One approval from any listed reviewer |
| Self-review | Allowed |
| Administrator bypass | Disabled |
| Allowed refs | Tags matching `v*-rc.*` only; branches are not allowed |

## Approval Procedure

1. Open the workflow run awaiting approval in **Actions**.
2. Review the tag, commit, validation results, and artifacts to be published.
3. Click **Review deployments** and select `prerelease`.
4. Select **Approve and deploy** to proceed, or **Reject** to stop the run.
