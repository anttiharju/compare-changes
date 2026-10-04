# Release notes

The GitHub release uses the extended description of the merge commit as handwritten release notes.
The workflow reads the commit message from the push event, not the pull request body.
The commit subject is not part of the notes.

An empty description, whitespace-only text, or the unchanged pull request title keeps GitHub's generated notes.
No release-note files are necessary.

The workflow preserves the handwritten text, including indentation and newlines.
It appends `---` and GitHub's generated **Full Changelog** link.
The link remains the last line.
The standalone action releases use the same notes.

If GitHub provides no changelog link, the step fails without replacing the generated notes.
The release stays a draft.

## At merge time

1. In the merge dialog, replace **Extended description** with the release notes.
2. Keep the notes out of the **Commit message** field.
3. Select **Confirm merge**.

Example description:

```markdown
- find-changes-action: Keep checkout credentials disabled.
- Fix checksum validation on macOS.
```

For generated notes, leave **Extended description** empty.
