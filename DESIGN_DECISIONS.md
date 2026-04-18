# Design Decisions (ADR)

## 💡 Philosophy
This project uses Architectural Decision Records (ADR) to track significant design choices.

## 📝 Decision Log

### ADR-001: Initial Scaffolding
- **Status:** Accepted
- **Context:** Bootstrapped using MLOps Meta-Repo.
- **Decision:** Use Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch for the core implementation to balance performance and safety.
- **Consequences:** Provides a solid foundation for vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync..

---
*Add new decisions above this line using the standard ADR format.*
