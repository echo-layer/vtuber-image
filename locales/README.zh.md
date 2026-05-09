<div align="center">

# vtuber-image

**ComfyUI REST wrapper and civitai loader for the vtuber-* program. Takes a persona spec from vtuber-commons and generates a rig-ready character image through curated workflow.json templates. Not a ComfyUI fork — ComfyUI runs as an external engine, called over REST.**

[![CI](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/ci.yml)
[![Security](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml/badge.svg)](https://github.com/echo-layer/vtuber-image/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-293-dea584.svg) ![Python LOD](https://img.shields.io/badge/Python_LOD-215-3776AB.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-678-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](https://www.rust-lang.org/) [![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)](https://www.python.org/) [![PyTorch](https://img.shields.io/badge/PyTorch-EE4C2C?logo=pytorch&logoColor=white)](https://pytorch.org/)

</div>

---

[ [English](../README.md) | [ภาษาไทย](./README.th.md) | [日本語](./README.ja.md) | 简体中文 ]

vtuber-image 将 ComfyUI 包装在 typed gRPC 入口之后 —— 并非 ComfyUI 分支,而是通过 REST 调用外部 ComfyUI 的封装。每次 GenerationRequest (携带来自 vtuber-commons 的 persona ID 以及可选 override) 都加载预定的 workflow.json 模板喂入 ComfyUI,并返回带 provenance 元数据的生成图。所需模型通过 allowlist 加载器从 civitai 下载,每次加载前按 vtuber-commons 核验 hash、license 与 NSFW 标签 —— 防止 pickle/LoRA 供应链攻击。

## ✨ 特性 (Features)
- 🚀 **Rust gRPC 前端 + Python ComfyUI REST 客户端 —— 接收 GenerationRequest (persona ID 与 override) 并返回带 provenance 元数据的图像**
- 🛡️ **civitai 加载器 + allowlist 强制 (下载/加载前用 vtuber-commons 校验 hash、license 与 NSFW)**
- 📊 **策划过的 workflow.json 模板库 —— 针对可爱动漫角色生成调参的 Flux dev q8 与 SDXL 工作流**

## 🛠️ 快速开始 (Quick Start)
```bash
# 安装 Rust toolchain、Python 3.12+ 以及运行中的 ComfyUI 实例 (显存至少 14 GB),.env 设置 COMFYUI_URL 指向 ComfyUI REST 端点,CIVITAI_TOKEN 用于鉴权下载,然后 cargo build && pip install -r python/requirements.txt,再 cargo run 在 8083 端口启动
```

## 🗺️ 导航 (Navigation)
- 🏗️ **[架构 (Architecture)](../ARCHITECTURE.md)**
- 📅 **[路线图 (Roadmap)](../ROADMAP.md)**
- 🤝 **[贡献 (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[项目结构 (Structure)](../STRUCTURE.tree)**

## ⚖️ 许可证 (License)
[MIT](../LICENSE)
