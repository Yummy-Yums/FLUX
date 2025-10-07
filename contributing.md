# Contributing to FLUX

Thank you for your interest in contributing to FLUX!
We'd love for you to contribute and help make it better. Nothing fancy as this is a project for learning/educational purposes

- [Issues and Bugs](#found-a-bug)
- [Submission Guidelines](#submitting-an-issue)
- [Code Quality](#-code-quality)
- [Links to documentation](#links-to-documentation)

## Found a bug?

If you find a bug in the source code, you can help us by [submitting an issue](#a-namesubmita-submission-guidelines)

Even better, you can [submit a Pull Request](#a-namesubmit-pra-submitting-a-pull-request-pr) with a fix.

## <a name="submit"></a> Submission Guidelines

### Submitting an Issue

Before you submit an issue, please search the issue tracker. An issue for your problem might already exist and the discussion might inform you of workarounds readily available.

You can file new issues by selecting a `Bug Report` template on the Issues submission page.

### <a name="submit-pr"></a> Submitting a Pull Request (PR)

Before you submit your Pull Request (PR) consider the following guidelines:

1. Search the repository for an open or closed PR that relates to your submission.
   You don't want to duplicate existing efforts.

2. Be sure that an issue describes the problem you're fixing, or documents the design for the feature you'd like to add.
   Discussing the design upfront helps to ensure that we're ready to accept your work.

3. [Fork](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo) the corresponding repo.

4. In your forked repository, make your changes in a new git branch:

     ```shell
     git checkout -b my-fix-branch main
     ```
5. Ensure that all tests pass.

6. Commit your changes using a descriptive commit message that follows our [commit message conventions](#commit-message-type).
   Adherence to these conventions is necessary because release notes are automatically generated from these messages.

     ```shell
     git commit -m <message>
   
     git commit -m "fix: bug a fixed"
     ```
   Note: the optional commit `-a` command line option will automatically "add" and "rm" edited files.

7. Push your branch to GitHub:

   ```shell
   git push origin my-fix-branch
   ```

8. In GitHub, send a pull request to `main`.

That's it! Thank you for your contribution!

## Links to Documentation

Please find documentation [`readme.md`](readme.md)

## 📋 Code Quality

- Follow Rust naming conventions and idioms
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Ensure all tests pass: [testing](#testing)

## Commit Message type

Must be one of the following:

* **build**: Changes that affect the build system or external dependencies
* **docs**: Documentation only changes
* **feat**: A new feature
* **fix**: A bug fix
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **test**: Adding missing tests or correcting existing tests

## Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test file
cargo test --test your_test_file