# lovyangfx-fonts-efont-cn

[![crates.io](https://img.shields.io/crates/v/lovyangfx-fonts-efont-cn.svg)](https://crates.io/crates/lovyangfx-fonts-efont-cn)
[![docs.rs](https://docs.rs/lovyangfx-fonts-efont-cn/badge.svg)](https://docs.rs/lovyangfx-fonts-efont-cn)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts) 的**随包字库数据**：eFont CN 点阵字体，5 个字号（10 / 12 / 14 / 16 / 24 px）× 4 个样式（常规 / **粗体** / **斜体** / **粗斜体**），u8g2 wire format，用 `lgyf-gen` 从 LovyanGFX 的 C 源码中提取。

主 crate 刻意"代码开源、数据自备"（规避包体上限与版权混装）；本包补上开箱即用的那一半——`cargo add` 即用，同时主 crate 的"自备任意 blob"能力原样保留。

## 为什么值得装

- **开箱即用**：不用 clone LovyanGFX、不用跑提取工具，一行 feature 就能出字。
- **中文粗体 / 斜体是稀缺能力**：u8g2 官方目录（1998 个字体）里没有任何真斜体，主字体族（文泉驿 / unifont / GB 宋体）连真粗体都没有；本包是同族四样式的完整矩阵。
- **LovyanGFX 视觉兼容**：blob 与上游 `lgfx_efont_cn.c` 数组逐字节一致——想在 Rust 固件里得到和 C++ LovyanGFX 项目相同的字形，用这个。
- **回归测试默认可跑**：主 crate 的真库回归需要本地检出 LovyanGFX；本包把数据带在身上，任何人 `cargo test` 都能跑完全量（金标准长度 + ~23.9k 码位逐个解码）。
- **按需链接**：每个字体一个 feature，没启用的字体不会进固件。

## 用法

本包 0.2 起是**零依赖纯数据**（只有 blob 常量，不带解码器）。两条路：

**路线一（推荐）：主 crate 一行开箱**——`fonts` 模块经 optional 依赖桥接本包：

```toml
[dependencies]
lovyangfx-fonts = { version = "0.2", features = ["efont-cn-14"] }
```

```rust
let font = lovyangfx_fonts::fonts::efont_cn_14();   // Font<'static>，零拷贝
assert!(font.has_glyph('你'));
```

**路线二：直接用本包**——自己拿 blob 建解码器（比如要把字库烧到外部存储自管加载）：

```toml
[dependencies]
lovyangfx-fonts = "0.2"                          # 解码器
lovyangfx-fonts-efont-cn = "0.2"                 # 默认带全部 20 个变体
# 只要一个字体：
# lovyangfx-fonts-efont-cn = { version = "0.2", default-features = false, features = ["efont-cn-14"] }
```

```rust
use lovyangfx_fonts::Font;

let font = Font::new(lovyangfx_fonts_efont_cn::EFONT_CN_14_BLOB).unwrap();
assert!(font.has_glyph('你'));
let width = font.text_width("你好AI");
font.for_each_pixel("你好AI", 232, |x, y| {
    /* draw_pixel(x, y); */
});
```

数据清单：`EFONT_CN_10_BLOB … EFONT_CN_24_BI_BLOB`（20 个，按 feature 门控）；字体访问函数
`efont_cn_10() … efont_cn_24_bi()` 在主 crate 0.2 的 `fonts` 模块里，feature 名两边一致。

## 字体清单

| 字号 | 常规 | 粗体 | 斜体 | 粗斜体 |
|---|---|---|---|---|
| 10 px | 158,417 B | 159,551 B | 170,008 B | 174,904 B |
| 12 px | 213,444 B | 211,952 B | 232,931 B | 235,895 B |
| 14 px | 262,233 B | 267,590 B | 288,018 B | 293,038 B |
| 16 px | 318,199 B | 320,446 B | 346,363 B | 357,031 B |
| 24 px | 550,804 B | 564,226 B | 576,487 B | 601,696 B |

共 20 个变体 6.3 MB；feature 名即 `efont-cn-{字号}` 与 `efont-cn-{字号}-{b|i|bi}`，另有 `efont-cn`（五个常规）与 `efont-cn-all`（全部，默认启用）。字形约 7.5k（GB2312 常用汉字 + ASCII + 符号），缺字按 `max_width()` 兜底（与 LovyanGFX 行为一致）。

## 与 u8g2-fonts 的关系

诚实对比，各取所需：

| | [u8g2-fonts](https://crates.io/crates/u8g2-fonts) | 本包 + 主 crate |
|---|---|---|
| 定位 | embedded-graphics 文本渲染器 + 内置字体目录（1998 个） | 解码器 + 数据包分离 |
| 自备字体 | 目录锁定，无裸字节入口 | `Font::new(任意 blob)` |
| 依赖 | 依赖 embedded-graphics | 解码器零依赖、`#![no_std]`、`forbid(unsafe)` |
| 中文粗/斜体 | 无（仅一个美术字粗体） | 5 字号 × 粗 / 斜 / 粗斜 |
| 数据许可 | 字体数据非 MIT/Apache（README 有提醒） | 同样分层：代码 MIT / 数据随上游声明（见 NOTICE） |

要"选个字体就渲染"且已在用 embedded-graphics → 用 u8g2-fonts，它很成熟。要自备字体、子集裁剪、不要 e-g 依赖、要中文强调样式 → 这边。

## 许可与来源

- crate 代码：MIT（见 [LICENSE](LICENSE)）。
- 字体数据：/efont（Electronic Font Open Laboratory，BSD 风格）经 LovyanGFX（FreeBSD）转换为 u8g2 数组，**数据不因本包改标 MIT**。两份上游声明逐字随包分发于 [licenses/](licenses/)，义务清单见 [NOTICE](NOTICE)——你把本包字体烧进固件再分发时，同样需要随附这两份声明。
- 重新生成（可复现）：

  ```sh
  lgyf-gen <你的LovyanGFX检出>/src/lgfx/Fonts/efont/lgfx_efont_cn.c -o fonts --only "lgfx_efont_cn_*"
  sha256sum fonts/*.bin > fonts/SHA256SUMS
  ```

  产物应与本包 `fonts/` 下的文件及 [fonts/SHA256SUMS](fonts/SHA256SUMS) 逐字节一致。

## 开发

```sh
cargo test                 # 全量回归（默认 = 全部字体）
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
cargo run --example text_metrics
```

CI（`.github/workflows/ci.yml`）跑 fmt / clippy 双模式 / 全量测试 / 包体积守卫（crates.io 上限 10 MB）。

## 发布前检查清单

- [ ] 确认 `Cargo.toml` 的 `repository` 指向实际 GitHub 仓库
- [ ] `cargo package` 体积 < 10 MB（当前压缩后 ≈ 5 MB，解包 6.0 MB）
- [ ] `cargo publish --dry-run`
- [ ] `cargo publish`

## 相关项目

- [`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts) — 本包配套的 no_std 解码器与 `lgyf-gen` 提取工具
- [LovyanGFX](https://github.com/lovyan03/LovyanGFX) — 字体数组来源（u8g2 格式）
- [/efont](https://osdn.net/projects/efont/) — 原始点阵字体

## License

代码 MIT © 2026 wangguanghua2099；字体数据许可见 [NOTICE](NOTICE) 与 [licenses/](licenses/)。
