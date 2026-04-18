# Project Strategy

## 🎯 Strategic Intent
Our goal is to build `vtuber-image` as a leader in the Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch ecosystem by focusing on:
**ComfyUI as external engine (REST). civitai loader enforces allowlist from vtuber-commons (hash, license, NSFW). Workflow.json templates ship alongside persona schemas so base image and persona stay version-locked.**

## 🗺️ Execution Pillars
1. **Rapid Prototyping:** Iterating quickly while maintaining core architectural integrity.
2. **Community Feedback:** Using user insights to drive the roadmap.
3. **Automation First:** Every repetitive task should be a script or a workflow.

## 📈 Success Metrics
- **Performance:** Achievement of benchmarks defined in `ARCHITECTURE.md`.
- **Stability:** Passing all tests in `cargo test && cargo clippy --all-targets -- -D warnings && pytest python/`.
- **Adoption:** Clear documentation and easy onboarding per `README.md`.
