# Project Vision

## 🌟 Mission Statement
**AI-only character image pipeline — given a persona spec from vtuber-commons, produce a rig-ready base image with zero manual ComfyUI clicking, backed by a community-safe civitai consumption ledger.**

## 🎯 Primary Objectives
- **Objective 1:** AI-only character image pipeline — given a persona spec from vtuber-commons, produce a rig-ready base image with zero manual ComfyUI clicking
- **Objective 2:** Community-safe civitai consumption — every downloaded model tracked by hash, license, and provenance in an auditable ledger

## 🔭 Long-term Impact
`vtuber-image` aims to solve the following problem:
vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync.

By leveraging `Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch`, we ensure that our solution is not only functional but also future-proof and high-performing.
