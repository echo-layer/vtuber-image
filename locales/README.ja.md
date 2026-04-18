<div align="center">

# vtuber-image

**ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.**

[![CI](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

<!-- Language Badges: Synchronize with English README -->
[AI: Generate the same individual language badges here]

<!-- LOD Badges: Synchronize with English README -->
[AI: Generate the same LOD badges here]

</div>

---

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | 日本語 | [简体中文](./README.zh.md) ]

> [AI: TRANSLATE the professional tagline/description into JAPANESE here]
vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync.

## ✨ 特徴 (Features)
> [AI: TRANSLATE all 3 Features into JAPANESE here]

## 🛠️ クイックスタート (Quick Start)
> [AI: TRANSLATE getting_started_instructions into JAPANESE here]

## 🗺️ ナวิゲーション (Navigation)
- 🏗️ **[アーキテクチャ (Architecture)](../ARCHITECTURE.md)**
- 📅 **[ロードマップ (Roadmap)](../ROADMAP.md)**
- 🤝 **[貢献する (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[プロジェクト構造 (Structure)](../STRUCTURE.tree)**

## ⚖️ ライセンス (License)
[MIT](../LICENSE)
