---
name: lint-format-check
type: knowledge
version: 1.0.0
agent: CodeActAgent
triggers: []
---

# Lint and Format Check

This microagent ensures that lint and format checks are performed before committing changes.

## Usage

When you are about to commit changes to the repository, make sure to run lint and format checks first.

## Instructions

Before creating a commit:

1. Run lint checks to ensure code quality:
   - For Rust projects: Run `cargo clippy` or check for available lint commands in the Makefile
   - For other projects: Run appropriate linters (e.g., eslint, pylint, etc.)

2. Run format checks to ensure consistent code style:
   - For Rust projects: Run `cargo fmt --check` or `rustfmt`
   - For other projects: Run appropriate formatters (e.g., prettier, black, etc.)

3. If any checks fail, fix the issues before committing.

4. Only proceed with the commit once all lint and format checks pass.

## Example

```bash
# Run lint checks
cargo clippy

# Run format checks
cargo fmt --check
```
