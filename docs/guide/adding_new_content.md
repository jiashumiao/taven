# Guide to Adding New Content to the Project

This guide aims to standardlize the operation process for adding new content to the `Taven` project, ensuring code quality, functional compatibility, and consistency of project structure. Whether you are adding new crates, modules, utility function, feature functionalities, or fixing extension - type bugs, you must follow the following core principles and steps to reduce collaboration costs and improve development efficiency.

## Preparation

Firstly, you need to create a new feature branch based on `main` brach, with the naming format following the specification of `feature/feature - name` or `fix/issue - description`. Secondly, before development, you should confirm that the new content is compatible with the existing architecture of the coding phase, you must stricty comply with the Rust Code Style Guide (such as the formatting requirements of `rustfmt`), and write corresponding unit tests and integration tests for the new logic to ensure that the test coverage does not fall below the project's baseline value. After the development is completed, execute `cargo build` and `cargo test` to verify the build and test pass rates. Then submit the code and initiate a Pull Request, clearly marking the modified content, design ideas, and test results. After the code review is passed, it can be merged into the main branch.


