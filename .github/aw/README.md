# Dependabot Release Labels

This workflow uses [GitHub Agentic Workflows](https://docs.github.com/en/copilot/concepts/agents/about-github-agentic-workflows) with the supported GitHub Copilot engine.
GitHub Agentic Workflows is in public preview.

## Behavior

[../workflows/dependabot-release.md](../workflows/dependabot-release.md) contains the workflow source.
[../workflows/dependabot-release.lock.yml](../workflows/dependabot-release.lock.yml) contains the output from the official `gh-aw` v0.89.21 compiler.
[actions-lock.json](actions-lock.json) records the action pin that the compiler resolves.

`zizmor` v1.25.2 repairs three compiler-generated substitutions after compilation.
These repairs pass integrity-policy lists through environment variables instead of direct shell interpolation.
The policy values and repository restrictions stay unchanged.

The workflow runs after `Plan` completes on a `dependabot/**` branch.
Only an open Dependabot PR in this repository, with the same head SHA and the default base branch, qualifies.
An existing `major-release`, `minor-release`, or `patch-release` label prevents changes.

The agent estimates the effect on compare-changes, not the version increment of the dependency.
The `release-type` safe output accepts one label from this list:

- `major-release`: A public interface or supported behavior breaks.
- `minor-release`: A backward-compatible public feature appears.
- `patch-release`: Shipped dependencies change without an identified public interface break or new feature.

The prompt requests `noop` when no release is necessary or the evidence is incomplete.
This output leaves all labels unchanged.
A successful label write removes only the legacy `dependencies` release alias.
The workflow does not change the separate explicit-label policy from issue #272.

## Security

The agent has read-only repository permissions.
Its GitHub tools can read files and PR data only in this repository.
The configuration disables shell tools, file edits, the CLI proxy, and automatic custom instructions.
The agent does not receive a repository checkout.

Compiler setup jobs read trusted configuration from the default branch.
The workflow does not execute PR code or download artifacts from `Plan`.
All actions use full commit SHAs, and compiler-selected containers use image digests.

Only the `release_type` job can run with write permission.
It requires successful agent execution and successful threat detection.
It accepts exactly one approved label and derives the PR number from the event, not the agent output.
It makes another eligibility check before the write and preserves existing explicit release labels.
It cannot create labels, comments, approvals, merges, or releases.

The compiler retains `issues: write` on its conclusion job and ignores permission overrides for that job.
The source disables that job with `if: "false"`.
This prevents the default conclusion reports from publishing issues or comments.

GitHub does not make the eligibility read and label writes atomic.
A concurrent manual label change or new PR commit can occur between those API calls.
Labels that the workflow adds with `GITHUB_TOKEN` do not trigger another workflow run.

## Prerequisites

The repository needs GitHub Actions and permission to use the pinned official actions on GitHub-hosted runners.
The three approved release labels must already exist.
The compiled workflow and its source must exist on the default branch before the `workflow_run` trigger works.

The engine requires a repository Actions secret named `COPILOT_GITHUB_TOKEN`.
This secret contains a fine-grained personal access token with the account-level **Copilot Requests** permission.
The token owner needs an active GitHub Copilot plan and permission to use Copilot CLI.
The token does not need repository write permissions.
The engine and threat detection use this token for Copilot requests.

Repository API calls use the job-scoped `GITHUB_TOKEN`.
This configuration does not use organization-billed `copilot-requests: write` authentication.
The default `GITHUB_TOKEN` alone cannot authenticate the engine in this configuration.
Missing or expired engine credentials prevent label writes.

The [official Copilot authentication guide](https://github.github.com/gh-aw/reference/auth/#copilot_github_token) describes token access and organization policies.

## Update the Workflow

1. Install the pinned official compiler with the authenticated GitHub CLI:

   ```sh
   gh extension install github/gh-aw --pin v0.89.21
   ```

2. Edit [../workflows/dependabot-release.md](../workflows/dependabot-release.md).
3. From the repository root, compile the workflow in strict mode:

   ```sh
   gh aw compile dependabot-release --strict
   ```

4. From the repository root, apply the generated security repairs:

   ```sh
   zizmor --fix=all --no-progress .github/workflows/dependabot-release.lock.yml
   ```

5. Format the generated workflow:

   ```sh
   prettier --write .github/workflows/dependabot-release.lock.yml
   ```

6. Run the workflow linter:

   ```sh
   actionlint .github/workflows/dependabot-release.lock.yml
   ```

7. Run the security linter without the repair option:

   ```sh
   zizmor --no-progress .github/workflows/dependabot-release.lock.yml
   ```

8. Commit the source, compiled workflow, and action pins together.

Do not edit the compiled workflow by hand.
If the compiler version changes, examine its generated permissions and the disabled conclusion job before deployment.
