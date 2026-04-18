<div align="center">

# vtuber-image

**ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.**

[![CI](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-0-dea584.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-0-3776AB.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-0-3776AB.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-0-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](./) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](./) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](./) [![PyTorch](https://img.shields.io/badge/PyTorch-EE4C2C?logo=pytorch&logoColor=white)](./)

</div>

---

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | [日本語](./README.ja.md) | 简体中文 ]

> [AI: TRANSLATE the professional tagline/description into SIMPLIFIED CHINESE here]
vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync.

## ✨ 特性 (Features)
> [AI: TRANSLATE all 3 Features into SIMPLIFIED CHINESE here]

## 🛠️ 快速开始 (Quick Start)
> [AI: TRANSLATE getting_started_instructions into SIMPLIFIED CHINESE here]

## 🗺️ 导航 (Navigation)
- 🏗️ **[架构 (Architecture)](../ARCHITECTURE.md)**
- 📅 **[路线图 (Roadmap)](../ROADMAP.md)**
- 🤝 **[贡献 (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[项目结构 (Structure)](../STRUCTURE.tree)**

## ⚖️ 许可证 (License)
[MIT](../LICENSE)
