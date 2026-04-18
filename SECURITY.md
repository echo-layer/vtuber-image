# Security Policy

## 🛡️ Commitment
We take the security of `vtuber-image` seriously. This project aims for a **Medium** security standard.

## 📢 Reporting a Vulnerability
Please do not report security vulnerabilities through public GitHub issues. Instead, send a detailed report to:
**mr.bt1590@gmail.com**

### What to include:
- A description of the vulnerability.
- Steps to reproduce (PoC).
- Potential impact.

## 🔐 Security Protocols
Every civitai model fetched MUST be verified against the allowlist in vtuber-commons before load — check file hash, license tags, and NSFW flags. Block pickle payloads by default (scan safetensors headers, reject .pt / .bin from untrusted sources). Workflow.json MUST only reference approved custom_nodes — reject any workflow that loads arbitrary Python at runtime. Log every model download with hash, civitai URL, and allowlist decision for audit.

- **Dependency Management:** Regularly scan for vulnerable packages.
- **CI/CD Security:** Mandatory automated security scans are integrated into `.github/workflows/security.yml`.
- **Disclosure:** We follow a responsible disclosure timeline.
