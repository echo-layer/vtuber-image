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

[ [English](../README.md) | ภาษาไทย | [日本語](./README.ja.md) | [简体中文](./README.zh.md) ]

vtuber-image ครอบ ComfyUI ด้วย typed gRPC surface — ไม่ใช่ fork ของ ComfyUI แต่เป็น wrapper ที่เรียก ComfyUI ภายนอกผ่าน REST ทุก GenerationRequest (persona ID จาก vtuber-commons + override) โหลด workflow.json template ที่ curate ไว้, ส่งเข้า ComfyUI, คืนรูปพร้อม provenance metadata — model ที่ต้องใช้ดาวน์โหลดจาก civitai ผ่าน allowlist loader ที่ verify hash, license, NSFW tag กับ vtuber-commons ก่อน load เสมอ กัน supply-chain attack แบบ pickle/LoRA

## ✨ ฟีเจอร์เด่น (Features)
- 🚀 **Rust gRPC frontend + Python ComfyUI REST client — รับ GenerationRequest (persona ID + override) คืนรูปพร้อม provenance metadata**
- 🛡️ **civitai loader พร้อม allowlist enforcement (hash, license, NSFW flag ตรวจกับ vtuber-commons ก่อน download และ load)**
- 📊 **Library ของ workflow.json template ที่ curate — Flux dev q8 และ SDXL workflow ปรับจูนสำหรับ cute anime character generation**

## 🛠️ เริ่มต้นใช้งาน (Quick Start)
```bash
# ติดตั้ง Rust toolchain, Python 3.12+ และ ComfyUI instance ที่ VRAM อย่างน้อย 14 GB ตั้ง COMFYUI_URL ใน .env ชี้ไปที่ ComfyUI REST, ตั้ง CIVITAI_TOKEN สำหรับ authenticated download แล้ว cargo build && pip install -r python/requirements.txt ก่อน cargo run ที่ port 8083
```

## 🗺️ การนำทาง (Navigation)
- 🏗️ **[สถาปัตยกรรม (Architecture)](../ARCHITECTURE.md)**
- 📅 **[แผนงาน (Roadmap)](../ROADMAP.md)**
- 🤝 **[การร่วมพัฒนา (Contributing)](../CONTRIBUTING.md)**
- 🌳 **[โครงสร้างโปรเจกต์ (Structure)](../STRUCTURE.tree)**

## ⚖️ ลิขสิทธิ์ (License)
[MIT](../LICENSE)
