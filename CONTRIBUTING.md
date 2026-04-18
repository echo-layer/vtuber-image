# Contributing to vtuber-image

We are committed to building a high-quality MLOps ecosystem. Thank you for your interest!

## 🚀 Workflow
1. **Fork** the repository and create your branch from `main`.
2. Ensure your code follows the **[PRINCIPLES.md](PRINCIPLES.md)**.
3. Use **[DESIGN_DECISIONS.md](DESIGN_DECISIONS.md)** to log any architectural changes.
4. Open a Pull Request using the **[PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md)**.

## ⚖️ Standards
- **Testing:** All features must include tests runnable via `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.
- **Quality:** Run `cargo build && pip install -r python/requirements.txt` and ensure a clean build.
- **Ethics:** Follow our **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)**.

## 🏷️ Issues
- Use **[bug_report.md](.github/ISSUE_TEMPLATE/bug_report.md)** for issues.
- Use **[feature_request.md](.github/ISSUE_TEMPLATE/feature_request.md)** for ideas.
