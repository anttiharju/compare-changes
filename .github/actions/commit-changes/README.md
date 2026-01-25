# Commit changes

Commits all changes as `github-actions[bot]` (can be overwritten with `committer` input) and uses [`github.actor`](https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context) as commit author to leave a record of automation origin.

Also has an output `changed` so following steps can be ran conditionally based on the output of this step.

Confirmed to work on the following events

- [`push`](https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#push)
- [`pull_request`](https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#pull_request)

## Example

```yml
name: Distribute
on:
  workflow_call:
    inputs:
      tag:
        required: true
        type: string
    secrets:
      ANTTIHARJU_BOT_ID:
        required: true
      ANTTIHARJU_BOT_PRIVATE_KEY:
        required: true

permissions:
  contents: write

jobs:
  homebrew-tap:
    name: Homebrew tap
    runs-on: ubuntu-24.04
    env:
      TAG: ${{ inputs.tag }}
    steps:
      - name: Checkout
        uses: actions/checkout@v5
      - name: Generate commit token
        id: generate-token
        uses: actions/create-github-app-token@v1
        with:
          app-id: ${{ secrets.ANTTIHARJU_BOT_ID }}
          private-key: ${{ secrets.ANTTIHARJU_BOT_PRIVATE_KEY }}
          repositories: homebrew-tap
      - name: Checkout
        uses: actions/checkout@v5
        with:
          repository: anttiharju/homebrew-tap
          token: ${{ steps.generate-token.outputs.token }}
          path: homebrew-tap
      - name: Download binary archives
        env:
          GH_TOKEN: ${{ github.token }}
          TAG: ${{ inputs.tag }}
        run: |
          gh release download "$TAG" --pattern '${{ github.event.repository.name }}-*64.tar.gz'
          ls -- *.tar.gz
      - name: Render formula template
        uses: anttiharju/actions/render-template@v0
        with:
          template: dist/brew/template.rb
          values: dist/brew/values.bash
          output: homebrew-tap/Formula/${{ github.event.repository.name }}.rb
      - name: Commit changes
        uses: anttiharju/actions/commit-changes@v0
        with:
          committer: "anttiharju[bot]"
          message: |
            Update ${{ github.event.repository.name }} formula to ${{ inputs.tag }}

            https://github.com/${{ github.repository }}/blob/${{ github.sha }}/.github/workflows/distribute.yml
          working-directory: homebrew-tap
```
