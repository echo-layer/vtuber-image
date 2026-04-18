# Engineering Principles

These principles guide the development and maintenance of `vtuber-image`.

## 🛠️ Core Architecture
- **Wrap, do not fork (ComfyUI is called over REST — we never diverge from upstream):** Our primary architectural guideline to ensure code remains clean and understandable.
- **Curated workflow templates over free-form prompts (character generation ships as versioned workflow.json):** Secondary principle focusing on the specific performance and safety needs of the Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch stack.

## ⚖️ Quality Standards
1. **Uncompromising Safety:** Every line of code must prioritize data integrity and memory safety.
2. **Predictable Performance:** Zero-cost abstractions are preferred over convenience if performance is impacted.
3. **Comprehensive Testing:** No feature is complete without an automated test suite runnable via `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.

## 🤝 Collaborative Values
- **Explicit over Implicit:** Code should be self-documenting and intent should be clear.
- **Incremental Excellence:** We value small, high-quality PRs over massive, complex changes.
