# Contributing to tsk

First off, thank you for considering contributing to tsk! It's people like you that make tsk such a great tool. We welcome contributions from everyone.

By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to see if the problem has already been reported. If you find an open issue that matches your problem, you can leave a comment with any additional information you have.

When you are creating a bug report, please include as many details as possible:

- Use a clear and descriptive title for the issue to identify the problem.
- Describe the exact steps which reproduce the problem in as much detail as possible.
- Describe the behavior you observed after following the steps and point out what exactly is the problem with that behavior.
- Explain which behavior you expected to see instead and why.
- Include the output of `tsk --help` and your Rust version (`rustc --version`).
- Note your OS and any other relevant environment details.

### Suggesting Enhancements

If you have an idea for a new feature or an improvement to an existing one, please create an issue with the following information:

- Use a clear and descriptive title for the issue to identify the suggestion.
- Provide a step-by-step description of the suggested enhancement in as much detail as possible.
- Explain why this enhancement would be useful to most users.
- List any alternative solutions or features you've considered.

### Pull Requests

We actively welcome your pull requests.

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code is formatted with `rustfmt`.
6. Issue that pull request!

## Development Setup

Please refer to the [Getting Started](README.md#getting-started) section in our README for instructions on how to set up the project locally.

## Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature-name`
3. Make your changes and commit them (see [Commit Messages](#commit-messages) below)
4. Push to your fork: `git push origin feat/your-feature-name`
5. Open a Pull Request against `main`

### Branch Naming Convention

Use the following prefixes for your branch names:

- `feat/` — New features (e.g., `feat/recurring-tasks`)
- `fix/` — Bug fixes (e.g., `fix/search-empty-results`)
- `docs/` — Documentation changes (e.g., `docs/update-readme`)
- `refactor/` — Code refactoring (e.g., `refactor/store-error-handling`)
- `test/` — Adding or updating tests (e.g., `test/filter-edge-cases`)

## Coding Standards

This project uses the following tools to maintain code quality:

- **Formatter:** [rustfmt](https://github.com/rust-lang/rustfmt) (default configuration)
- **Linter:** [clippy](https://github.com/rust-lang/rust-clippy)

Before submitting a PR, ensure your code passes all checks:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/).

Format: `<type>(<optional scope>): <description>`

Examples:

- `feat(store): add due date field to Task struct`
- `fix(command): handle missing task ID in edit command gracefully`
- `docs(readme): add usage examples for search and filter commands`
- `refactor(display): extract column width constants for format_task`
- `test(store): add edge case tests for delete_task with empty store`
- `feat(cli): add --sort flag to list command for priority ordering`
- `chore: update clap dependency to 4.7`

## Code Review Process

All submissions, including those by project members, require review. We use GitHub pull requests for this purpose. Consult [GitHub Help](https://help.github.com/articles/about-pull-requests/) for more information on using pull requests.

### What we look for:

- **Correctness** — Does the code do what it's supposed to do?
- **Tests** — Are there adequate tests? Do they pass?
- **Style** — Does the code follow the project's coding standards? Is it formatted with `rustfmt`?
- **Documentation** — Are any new features or changes documented?
- **Performance** — Are there any obvious performance concerns?
- **Error Handling** — Does the code return `Result` types instead of panicking?

## Recognition

Contributors who make significant contributions will be recognized in our README under the Acknowledgements section. We value every contribution, no matter how small!

---

Thank you for contributing! 🎉
