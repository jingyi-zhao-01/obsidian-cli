---
name: PR Validation
type: knowledge
version: 1.0.0
agent: CodeActAgent
---

This microagent helps ensure that the code changes pass all validation checks before creating a pull request.

## Usage

When you need to validate that the code passes all checks in the PR workflow, use this microagent.

## Tasks

Run the following validation steps to ensure the code is ready for submission:

1. **Format Check**: Run `make fmt-check` to ensure code is properly formatted
2. **Lint Check**: Run `make clippy-check` to run clippy lints

Both checks must pass before submitting a pull request.

## Running Validation

Execute these commands in the repository root:

```bash
# Format check
make fmt-check

# Lint check
make clippy-check
```

If either check fails, fix the issues and run the checks again until both pass.
