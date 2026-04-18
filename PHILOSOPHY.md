# Technical Philosophy

## 🧩 Architectural Mindset
The core of `vtuber-image` is built on the belief that software should be:
- **Resilient:** Handling failures gracefully.
- **Scalable:** Growing with the data volume.
- **Maintainable:** Easy for new contributors to understand.

## 🛠️ Implementation Choices
We prioritize `Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch` for its unique strengths in ComfyUI as external engine (REST). civitai loader enforces allowlist from vtuber-commons (hash, license, NSFW). Workflow.json templates ship alongside persona schemas so base image and persona stay version-locked..
