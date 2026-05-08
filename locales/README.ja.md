<div align="center">

# vtuber-image

**ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.**

[![CI](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-81-dea584.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-121-3776AB.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-268-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](https://www.rust-lang.org/) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](https://www.python.org/) [![PyTorch](https://img.shields.io/badge/PyTorch-EE4C2C?logo=pytorch&logoColor=white)](https://pytorch.org/)

</div>

---

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | 日本語 | [简体中文](./README.zh.md) ]

vtuber-image は ComfyUI を typed gRPC サーフェスでラップする — ComfyUI のフォークではなく、外部 ComfyUI を REST 経由で呼び出すラッパー。GenerationRequest (vtuber-commons の persona ID + オーバーライド) 毎にキュレートされた workflow.json テンプレートをロードして ComfyUI に渡し、provenance メタデータ付きで生成画像を返却。必要なモデルは allowlist ローダー経由で civitai からダウンロードし、vtuber-commons に対して hash・ライセンス・NSFW タグを毎回検証する — pickle/LoRA サプライチェーン攻撃を防止。

## ✨ 特徴 (Features)
- 🚀 **Rust gRPC フロントエンド + Python ComfyUI REST クライアント — GenerationRequest (persona ID + オーバーライド) を受け provenance メタデータ付き画像を返却**
- 🛡️ **civitai ローダー + allowlist 強制 (hash・ライセンス・NSFW フラグを vtuber-commons に照合してから download/load)**
- 📊 **キュレートされた workflow.json テンプレートライブラリ — Flux dev q8 と SDXL ベースワークフロー (可愛いアニメキャラ生成向けチューニング済み)**

## 🛠️ クイックスタート (Quick Start)
```bash
# Rust toolchain、Python 3.12+ と稼働中の ComfyUI インスタンス (VRAM 14 GB 以上) をインストール。.env の COMFYUI_URL を ComfyUI REST エンドポイントに、CIVITAI_TOKEN を認証ダウンロード用に設定後、cargo build && pip install -r python/requirements.txt、次いで cargo run でポート 8083 起動
```

## 🗺️ ナวิゲーション (Navigation)
- 🏗️ **[アーキテクチャ (Architecture)](../ARCHITECTURE.md)**
- 📅 **[ロードマップ (Roadmap)](../ROADMAP.md)**
- 🤝 **[貢献する (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[プロジェクト構造 (Structure)](../STRUCTURE.tree)**

## ⚖️ ライセンス (License)
[MIT](../LICENSE)
