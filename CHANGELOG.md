# Changelog

All notable changes to this project will be documented in this file.
The format follows [keep-a-changelog](https://keepachangelog.com/) and semver.

## 0.1.0 — 2026-09-27

### Added

- 20 bundled eFont CN blobs: sizes 10/12/14/16/24 px × regular/bold/italic/bold-italic,
  u8g2 wire format, extracted from LovyanGFX `lgfx_efont_cn.c` via `lgyf-gen`;
  `fonts/SHA256SUMS` pins the shipped bytes.
- One cargo feature per font (`efont-cn-14`, `efont-cn-14-b`, …), umbrella features
  `efont-cn` (five regular) and `efont-cn-all` (all 20, default); disabled fonts are
  not linked into the binary.
- `efont_cn_*()` zero-copy `Font` accessors and `EFONT_CN_*_BLOB` raw byte consts.
- Regression suite running by default on the bundled data: golden byte lengths,
  header sanity, ASCII + common-hanzi coverage, ~23.9k-codepoint decode sweep per
  font, baseline and line-clipping checks — the real-library regression that the
  main crate can only run with a local LovyanGFX checkout now runs everywhere.
- CI workflow (fmt / clippy in both feature modes / full tests / package-size guard)
  and a host-side `text_metrics` example.

### Verified

- all 20 blobs decode through `lovyangfx-fonts` 0.1 with identical behavior to the
  C++ `U8g2font` reader (per-codepoint sweep, no panics, bounded pixel counts);
- shipped file sizes match the upstream array declarations byte-for-byte;
- package size ≈ 5 MB compressed (6.0 MB unpacked), within the crates.io 10 MB cap (CI guard).
