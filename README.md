<div align="center">

# vtuber-image

**ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.**

[![CI](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-47-dea584.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-15-3776AB.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-15-3776AB.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-128-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](https://www.rust-lang.org/) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](https://www.python.org/) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](https://www.python.org/) [![PyTorch](https://img.shields.io/badge/PyTorch-EE4C2C?logo=pytorch&logoColor=white)](https://pytorch.org/)

</div>

---

[ English | [ภาษาไทย](./locales/README.th.md) | [日本語](./locales/README.ja.md) | [简体中文](./locales/README.zh.md) ]

vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync.

## ✨ Features

- 🚀 **Feature 1** — Rust gRPC frontend and Python ComfyUI REST client — receives GenerationRequest (persona ID and overrides), returns generated image with provenance metadata
- 🛡️ **Feature 2** — civitai API loader with allowlist enforcement (hash, license tags, NSFW flags checked against vtuber-commons before download and load)
- 📊 **Feature 3** — Curated workflow.json template library — Flux dev q8 and SDXL base workflows tuned for cute anime character generation, versioned alongside vtuber-commons persona schemas

## 🛠️ Quick Start

```bash
# Install Rust toolchain, Python 3.12+, and a running ComfyUI instance (see https://github.com/comfyanonymous/ComfyUI) with at least 14 GB VRAM. Set COMFYUI_URL in .env to point at the ComfyUI REST endpoint, set CIVITAI_TOKEN for authenticated downloads, then run cargo build and pip install -r python/requirements.txt before cargo run on port 8083.
```

## 🗺️ Navigation

- 🏗️ **[Architecture](ARCHITECTURE.md)** — Core design and components.
- 📅 **[Roadmap](ROADMAP.md)** — Project timeline and milestones.
- 🤝 **[Contributing](CONTRIBUTING.md)** — How to join and help.
- 🌳 **[Project Structure](STRUCTURE.tree)** — Full file map.

## ⚖️ License

[MIT](LICENSE)
