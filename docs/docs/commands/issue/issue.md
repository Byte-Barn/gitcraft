---
title: "gh-templates issue"
sidebar_label: "issue"
---

# Issue Templates

The `issue` subcommand provides functionality for managing GitHub issue templates. Issue templates help standardize bug reports, feature requests, and other types of issues in your repository.

## Usage

```bash
gh-templates issue <COMMAND>
```

## Available Commands

| Command  | Description                                               |
|----------|-----------------------------------------------------------|
| `add`    | Add one or more issue templates to the repository         |
| `list`   | List available issue templates                            |
| `preview`| Preview a specific issue template                         |
| `help`   | Print help for the issue subcommand or its subcommands    |

## Options

- `-h`, `--help`: Print help information

## Examples

### List Available Templates

```bash
gh-templates issue list
```

Sample output:

```
bug.yml - Bug Report Template
chore.yml - Chore Issue Template
community.yml - Report issues or suggestions related to community, collaboration, or project governance.
docs.yml - Report issues or suggest improvements related to documentation, guides, or help content.
dx.yml - Report issues that affect developers' experience
feature.yml - Suggest a new feature or improvement for a project.
refactor.yml - Refactor Issue template for GitHub
support.yml - Ask a question or request support (not for bugs or feature requests)
technical-debt.yml - Technical Debt Issue Template
test.yml - Report issues related to testing or quality assurance.
```

### Preview a Template

```bash
gh-templates issue preview bug
```

### Add Single Template

```bash
gh-templates issue add bug
```

### Add Multiple Templates

```bash
gh-templates issue add bug feature enhancement
```

### Show Help

```bash
gh-templates issue help
```

## Template Types

Issue templates typically include:

- **Bug Report Templates**: Structured forms for reporting bugs with steps to reproduce
- **Feature Request Templates**: Forms for proposing new features
- **Chore Templates**: For routine tasks or maintenance
- **Community Templates**: For community, collaboration, or governance issues
- **Documentation Templates**: For documentation or help content issues
- **Developer Experience (DX) Templates**: For issues affecting developer workflows
- **Refactor Templates**: For code refactoring suggestions
- **Support Templates**: For questions or support requests
- **Technical Debt Templates**: For tracking technical debt
- **Test Templates**: For testing or quality assurance issues
- **Enhancement Templates**: For suggesting improvements to existing features

## Output Location

By default, issue templates are saved to the `.github/ISSUE_TEMPLATE/` directory in your repository root. This follows GitHub's standard convention for issue templates.

## Next Steps

- [Add Issue Templates](./issue-add.md)
- [List Issue Templates](./issue-list.md)
- [Preview Issue Templates](./issue-preview.md)

Adding Labels to Your Repository

To ensure your repository uses the same labels as GitForge, follow these steps:

1. Copy Labels from GitForge
You can export labels from GitForge using the `gh label clone` command (requires the GitHub CLI):

```bash
gh label clone RafaelJohn9/gitforge --repo <your-username>/<your-repo>

```

This will duplicate all labels (like bug, feature, chore, documentation, etc.) into your repository.

2. Verify Labels

After cloning, check your repo’s Issues > Labels section on GitHub to confirm they match GitForge’s label set.

3. Maintain Consistency

Whenever GitForge updates its label set, rerun the clone command to sync updates.
