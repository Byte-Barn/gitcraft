# Contributing to This Project

Thank you for considering contributing!   
Your time and effort help us build and maintain a high-quality, reliable, and inclusive project.

---

## How to Contribute

1. **Fork the repository** and create your branch from `main`.
2. **Clone your fork** locally and install all dependencies.
3. **Create a feature branch** with a descriptive name (e.g. `fix/readme-typo` or `feat/add-logging`).
4. **Make your changes** — code, docs, or tests.
5. **Run tests locally** to ensure everything works.
6. **Commit your changes** following the commit message guidelines below.
7. **Push your branch** to your fork.
8. **Submit a Pull Request (PR)** with a clear and concise title and description.

---

## Code Style & Standards

- Follow the existing code and formatting style.
- Use consistent indentation, naming, and file organization.
- Keep functions small and focused.
- Add comments for complex logic.
- Prefer readability and clarity over cleverness.

---

## Running Tests Locally

This project uses **Cargo** for building and testing Rust code.

Before submitting a Pull Request, make sure all tests pass locally:

```bash
cargo build
cargo test
```

If applicable, you can run tests with additional flags for better output:

```bash
cargo test -- --nocapture
```

Please ensure new features or fixes include **relevant tests** and do not break existing ones.

---

## Commit Message Guidelines

Writing clear and meaningful commit messages helps everyone understand why a change was made.
Follow these conventions:

### Format

```
[type] (optional-scope): <short summary>
```

### Examples

```
feat(actions): added `--dry-run` to add command.
fix(docs): correct typo in README setup section
chore(ci): update GitHub Actions Rust version
refactor(api): simplify error handling
docs(contributing): add PR naming guidelines
```

### Tips

* Use the **imperative mood** (“add”, not “added” or “adds”).
* Limit the **subject line** to 72 characters.
* Include details in the commit body if necessary (why, what, how).
* Use prefixes like:

  * `feat:` for new features
  * `fix:` for bug fixes
  * `docs:` for documentation updates
  * `chore:` for maintenance or tooling changes
  * `refactor:` for restructuring without behavior change
  * `test:` for adding or updating tests

---

## Pull Request (PR) Naming & Description

Good PR titles make it easier for reviewers to understand the intent.

### Recommended PR title format:

```
[type]: short, descriptive summary
```

### Examples:

```
feat: implement user profile editing
fix: resolve panic in config loader
docs: improve contributing guide formatting
```

### PR Description Checklist

When opening a Pull Request:

* Explain **what** the change does.
* Explain **why** the change was needed.
* Include **screenshots** if applicable.
* Mention related **issue numbers** (e.g. “Closes #12”).
* Ensure your branch is **up to date** with `main`.
* Respond to **review comments** politely and promptly.

---

## Reporting Issues

When filing a new issue:

* **Search** existing issues to avoid duplicates.
* Use a **clear and descriptive title**.
* Provide steps to reproduce, expected vs. actual behavior.
* Include screenshots or logs if applicable.

---

## Community Standards

We are committed to creating an open, welcoming, and inclusive environment for everyone.

Please:

* Be respectful and constructive in discussions.
* Encourage collaboration and diversity of thought.
* Follow the [Code of Conduct](CODE_OF_CONDUCT.md).

---

Thank you for helping make this project better! 💪
Your contributions - whether code, documentation, or ideas - are all valuable.


