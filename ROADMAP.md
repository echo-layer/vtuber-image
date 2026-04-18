# Project Roadmap

## 📅 Timeline
Q2 2026 v0.1 (ComfyUI REST wrapper + SDXL workflow), Q3 2026 v0.2 (civitai loader + allowlist enforcement), Q4 2026 v0.5 (Flux dev workflow + persona-sync), Q1 2027 v1.0 (semver-locked)

## 🏁 Milestones
v0.1 SDXL wrapper, v0.2 civitai allowlist, v0.5 Flux dev + persona-sync, v1.0 stable

## 🚀 Future Vision
AI-only character image pipeline — given a persona spec from vtuber-commons, produce a rig-ready base image with zero manual ComfyUI clicking, backed by a community-safe civitai consumption ledger.

### Phase 1: Foundation
- [ ] Implement core Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch engine.
- [ ] Set up basic CI/CD in `.github/workflows/ci.yml`.

### Phase 2: Scale
- [ ] Optimize Curated workflow templates over free-form prompts (character generation ships as versioned workflow.json) implementations.
- [ ] Expand connector support.

### Phase 3: Excellence
- [ ] Full security audit per [SECURITY.md](SECURITY.md).
- [ ] Finalize production release.
