# Frequently Asked Questions (FAQ)

## ❓ General
**Q: What is vtuber-image?**
A: ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.

## 🛠️ Technical
**Q: How do I run tests?**
A: Use the command: `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.

**Q: Which languages are supported?**
A: Primarily Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch.

## 🤝 Contributing
**Q: How can I help?**
A: Check out [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get involved!
