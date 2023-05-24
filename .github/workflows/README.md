# Ziggurat Core CI/CD Workflows

This directory contains callable/reusable workflows that are used throughout the Ziggurat ecosystem. The documentation can be used as a reference for building your own CI/CD pipeline with Ziggurat.

## Workflow References

- [Build Ziggurat](./build-ziggurat.yml) - Compiles all Ziggurat unit tests with the provided features, and then uploads the resulting executable artifact.
- [Process Results](./process-results.yml) - Handles the post-processing and upload of the daily CI/CD results.
- [Check and Lint](./check-and-lint.yml) - Contains the basic checks and lints needed for a Rust project. Runs in a Nix development environment, details can be found [here](../../README.md#Nix).
