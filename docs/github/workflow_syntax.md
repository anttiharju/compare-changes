# Workflow syntax

From [github/docs/content/actions/reference/workflows-and-actions/workflow-syntax.md?plain=1#L1276-L1341@c1ca049](https://github.com/github/docs/blob/c1ca049106639bc87381aa16e40e35f743928246/content/actions/reference/workflows-and-actions/workflow-syntax.md?plain=1#L1276-L1341) which is covered by the **Creative Commons Attribution 4.0** license from [github/docs/LICENSE@c1ca049](https://github.com/github/docs/blob/c1ca049106639bc87381aa16e40e35f743928246/LICENSE) (also included in-repo in the adjacent [LICENSE](./LICENSE) file). Only automated formatting changes (e.g., prettier) and issues caught by `zensical build --strict` have been applied to the content below.

## Filter pattern cheat sheet

You can use special characters in path, branch, and tag filters.

- `*`: Matches zero or more characters, but does not match the `/` character. For example, `Octo*` matches `Octocat`.
- `**`: Matches zero or more of any character.
- `?`: Matches zero or one of the preceding character.
- `+`: Matches one or more of the preceding character.
- `[]` Matches one alphanumeric character listed in the brackets or included in ranges. Ranges can only include `a-z`, `A-Z`, and `0-9`. For example, the range`[0-9a-z]` matches any digit or lowercase letter. For example, `[CB]at` matches `Cat` or `Bat` and `[1-2]00` matches `100` and `200`.
- `!`: At the start of a pattern makes it negate previous positive patterns. It has no special meaning if not the first character.

The characters `*`, `[`, and `!` are special characters in YAML. If you start a pattern with `*`, `[`, or `!`, you must enclose the pattern in quotes. Also, if you use a [flow sequence](https://yaml.org/spec/1.2.2/#flow-sequences) with a pattern containing `[` and/or `]`, the pattern must be enclosed in quotes.

```yaml
# Valid
paths:
  - '**/README.md'

# Invalid - creates a parse error that
# prevents your workflow from running.
paths:
  - **/README.md

# Valid
branches: [ main, 'release/v[0-9].[0-9]' ]

# Invalid - creates a parse error
branches: [ main, release/v[0-9].[0-9] ]
```

For more information about branch, tag, and path filter syntax, see [`on.<push>.<branches|tags>`](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onpushbranchestagsbranches-ignoretags-ignore), [`on.<pull_request>.<branches|tags>`](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onpull_requestpull_request_targetbranchesbranches-ignore), and [`on.<push|pull_request>.paths`](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onpushpull_requestpull_request_targetpathspaths-ignore).

### Patterns to match branches and tags

| Pattern                                     | Description                                                                                                                                                                  | Example matches                                                                               |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `feature/*`                                 | The `*` wildcard matches any character, but does not match slash (`/`).                                                                                                      | `feature/my-branch`<br/><br/>`feature/your-branch`                                            |
| `feature/**`                                | The `**` wildcard matches any character including slash (`/`) in branch and tag names.                                                                                       | `feature/beta-a/my-branch`<br/><br/>`feature/your-branch`<br/><br/>`feature/mona/the/octocat` |
| `main`<br/><br/>`releases/mona-the-octocat` | Matches the exact name of a branch or tag name.                                                                                                                              | `main`<br/><br/>`releases/mona-the-octocat`                                                   |
| `'*'`                                       | Matches all branch and tag names that don't contain a slash (`/`). The `*` character is a special character in YAML. When you start a pattern with `*`, you must use quotes. | `main`<br/><br/>`releases`                                                                    |
| `'**'`                                      | Matches all branch and tag names. This is the default behavior when you don't use a `branches` or `tags` filter.                                                             | `all/the/branches`<br/><br/>`every/tag`                                                       |
| `'*feature'`                                | The `*` character is a special character in YAML. When you start a pattern with `*`, you must use quotes.                                                                    | `mona-feature`<br/><br/>`feature`<br/><br/>`ver-10-feature`                                   |
| `v2*`                                       | Matches branch and tag names that start with `v2`.                                                                                                                           | `v2`<br/><br/>`v2.0`<br/><br/>`v2.9`                                                          |
| `v[12].[0-9]+.[0-9]+`                       | Matches all semantic versioning branches and tags with major version 1 or 2.                                                                                                 | `v1.10.1`<br/><br/>`v2.0.0`                                                                   |

### Patterns to match file paths

Path patterns must match the whole path, and start from the repository's root.

| Pattern                                             | Description of matches                                                                                                                                                                        | Example matches                                                                         |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `'*'`                                               | The `*` wildcard matches any character, but does not match slash (`/`). The `*` character is a special character in YAML. When you start a pattern with `*`, you must use quotes.             | `README.md`<br/><br/>`server.rb`                                                        |
| `'*.jsx?'`                                          | The `?` character matches zero or one of the preceding character.                                                                                                                             | `page.js`<br/><br/>`page.jsx`                                                           |
| `'**'`                                              | The `**` wildcard matches any character including slash (`/`). This is the default behavior when you don't use a `path` filter.                                                               | `all/the/files.md`                                                                      |
| `'*.js'`                                            | The `*` wildcard matches any character, but does not match slash (`/`). Matches all `.js` files at the root of the repository.                                                                | `app.js`<br/><br/>`index.js`                                                            |
| `'**.js'`                                           | Matches all `.js` files in the repository.                                                                                                                                                    | `index.js`<br/><br/>`js/index.js`<br/><br/>`src/js/app.js`                              |
| `docs/*`                                            | All files within the root of the `docs` directory only, at the root of the repository.                                                                                                        | `docs/README.md`<br/><br/>`docs/file.txt`                                               |
| `docs/**`                                           | Any files in the `docs` directory and its subdirectories at the root of the repository.                                                                                                       | `docs/README.md`<br/><br/>`docs/mona/octocat.txt`                                       |
| `docs/**/*.md`                                      | A file with a `.md` suffix anywhere in the `docs` directory.                                                                                                                                  | `docs/README.md`<br/><br/>`docs/mona/hello-world.md`<br/><br/>`docs/a/markdown/file.md` |
| `'**/docs/**'`                                      | Any files in a `docs` directory anywhere in the repository.                                                                                                                                   | `docs/hello.md`<br/><br/>`dir/docs/my-file.txt`<br/><br/>`space/docs/plan/space.doc`    |
| `'**/README.md'`                                    | A README.md file anywhere in the repository.                                                                                                                                                  | `README.md`<br/><br/>`js/README.md`                                                     |
| `'**/*src/**'`                                      | Any file in a folder with a `src` suffix anywhere in the repository.                                                                                                                          | `a/src/app.js`<br/><br/>`my-src/code/js/app.js`                                         |
| `'**/*-post.md'`                                    | A file with the suffix `-post.md` anywhere in the repository.                                                                                                                                 | `my-post.md`<br/><br/>`path/their-post.md`                                              |
| `'**/migrate-*.sql'`                                | A file with the prefix `migrate-` and suffix `.sql` anywhere in the repository.                                                                                                               | `migrate-10909.sql`<br/><br/>`db/migrate-v1.0.sql`<br/><br/>`db/sept/migrate-v1.sql`    |
| `'*.md'`<br/><br/>`'!README.md'`                    | Using an exclamation mark (`!`) in front of a pattern negates it. When a file matches a pattern and also matches a negative pattern defined later in the file, the file will not be included. | `hello.md`<br/><br/>_Does not match_<br/><br/>`README.md`<br/><br/>`docs/hello.md`      |
| `'*.md'`<br/><br/>`'!README.md'`<br/><br/>`README*` | Patterns are checked sequentially. A pattern that negates a previous pattern will re-include file paths.                                                                                      | `hello.md`<br/><br/>`README.md`<br/><br/>`README.doc`                                   |
