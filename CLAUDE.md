# Project Intelligence & Operational Logic

This file is the operational core for Claude. Gemini CLI and Claude MUST follow these protocols to maintain project integrity.

## 🎯 Architectural Intent
- **Core Mission:** AI-only character image pipeline — given a persona spec from vtuber-commons, produce a rig-ready base image with zero manual ComfyUI clicking, backed by a community-safe civitai consumption ledger.
- **Primary Stack:** Rust, tonic, Python, ComfyUI, civitai API, Flux dev, SDXL, PyTorch
- **System Nature:** vtuber-image wraps ComfyUI behind a typed gRPC surface. It is explicitly NOT a ComfyUI fork — upstream ComfyUI runs as an external engine and we call it over REST. On each GenerationRequest (carrying a persona ID from vtuber-commons plus optional overrides), the service loads the matching curated workflow.json template, feeds it to ComfyUI, and returns the generated image with provenance metadata. Required models are downloaded from civitai through an allowlist loader that verifies hash, license, and NSFW tags against vtuber-commons before every load — preventing pickle/LoRA supply-chain attacks. Characters ship as versioned workflow.json templates alongside vtuber-commons persona schemas, so base image and persona stay in sync.

## 🧬 Automated Lifecycle Management
1. **Research Sync:** When `./scripts/update_notebookLM.sh` is executed:
   - You MUST update `DESIGN_DECISIONS.md` with new ADRs found in research.
   - **Constraint:** Maintain a rolling log of the **latest 10 ADRs**.
2. **PR Creation Protocol:** When instructed to create a Pull Request:
   - **Summarize:** Analyze all commit messages since the last merge to `main`.
   - **Template:** Read `.github/PULL_REQUEST_TEMPLATE.md` and populate it with:
     - Detailed description of changes.
     - Linked Issue ID (search for keywords like "fixes #123").
     - Automated Labels (e.g., `feat`, `fix`, `docs`).
   - **Assign:** Automatically set the current developer as the Assignee.
3. **Pre-Commit Action:** Before every commit, you MUST:
   - Run `tree -a -I 'node_modules|.git|target' > STRUCTURE.tree`.
   - Trigger stack-specific formatting (e.g., `cargo fmt`).
   - Run `pre-commit run --all-files` if available.

## 🛠️ Tooling & Standards
- **Cross-Repo Constraints:** Claude and Gemini do NOT have permission to modify other `vtuber-*` repositories directly. If a change is required in a sibling repository (e.g., `vtuber-commons`), you MUST create a GitHub Issue in that repository describing the need. Close the issue only once the corresponding task is complete.
- **Translation:** All technical specifications are English. `locales/` MUST be kept in sync and translated for users documentation.
- **Workflow Mastery:** Use `/superpower:executing-plans` for feature work.
- **Automation:** Refer to `.github/workflows/pr_automation.yml` for server-side PR handling.

## 📂 Template Inventory
You manage: ARCHITECTURE.md, ROADMAP.md, CONTRIBUTING.md, DESIGN_DECISIONS.md, STRUCTURE.tree, SECURITY.md, LICENSE.md, FAQ.md, GOVERNANCE.md, SUPPORT.md, TROUBLESHOOTING.md, PHILOSOPHY.md, MANIFESTO.md, and `locales/README.{th,ja,zh}.md`.
