# lovyangfx-fonts-efont-cn

[![crates.io](https://img.shields.io/crates/v/lovyangfx-fonts-efont-cn.svg)](https://crates.io/crates/lovyangfx-fonts-efont-cn)
[![docs.rs](https://docs.rs/lovyangfx-fonts-efont-cn/badge.svg)](https://docs.rs/lovyangfx-fonts-efont-cn)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Bundled font data** for [`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts): eFont CN bitmap fonts, 5 sizes (10 / 12 / 14 / 16 / 24 px) × 4 styles (regular / **bold** / **italic** / **bold-italic**), u8g2 wire format, extracted from LovyanGFX' C sources with `lgyf-gen`.

The main crate is deliberately "code open, data bring-your-own" (keeping the package small and the licensing clean); this package ships the out-of-the-box half — `cargo add` and go — while the main crate's "any blob of your own" capability stays intact.

## Why this crate

- **Out of the box**: no LovyanGFX checkout, no extraction tool — enable a feature and render.
- **CJK bold / italic is rare**: u8g2's own catalog (1998 fonts) has no true italic at all, and its main CJK families (WenQuanYi, unifont, GB song) have no true bold. This is a full 4-style matrix of one family.
- **LovyanGFX-compatible glyphs**: the blobs are byte-identical to the upstream `lgfx_efont_cn.c` arrays — same pixels as a C++ LovyanGFX project.
- **Regression tests run by default**: the main crate's real-library tests need a local LovyanGFX checkout; this package carries the data, so `cargo test` runs the full sweep (golden byte lengths + ~23.9k codepoints decoded one by one) everywhere.
- **Link only what you use**: one feature per font; disabled fonts never reach the binary.

## Usage

```toml
[dependencies]
lovyangfx-fonts = "0.1"                          # the decoder
lovyangfx-fonts-efont-cn = "0.1"                 # all 20 variants by default
# Just one font:
# lovyangfx-fonts-efont-cn = { version = "0.1", default-features = false, features = ["efont-cn-14"] }
```

```rust
use lovyangfx_fonts_efont_cn::efont_cn_14;

let font = efont_cn_14();                  // Font<'static>, zero-copy
assert!(font.has_glyph('你'));
let width = font.text_width("你好AI");
font.for_each_pixel("你好AI", 232, |x, y| {
    /* draw_pixel(x, y); */
});
# let _ = width;
```

Full API (one pair per variant): `efont_cn_10() … efont_cn_24_bi()` return a `lovyangfx_fonts::Font<'static>`; `EFONT_CN_10_BLOB … EFONT_CN_24_BI_BLOB` expose the raw bytes (e.g. for flashing to external storage or custom loading).

## Font catalog

| Size | Regular | Bold | Italic | Bold-italic |
|---|---|---|---|---|
| 10 px | 158,417 B | 159,551 B | 170,008 B | 174,904 B |
| 12 px | 213,444 B | 211,952 B | 232,931 B | 235,895 B |
| 14 px | 262,233 B | 267,590 B | 288,018 B | 293,038 B |
| 16 px | 318,199 B | 320,446 B | 346,363 B | 357,031 B |
| 24 px | 550,804 B | 564,226 B | 576,487 B | 601,696 B |

20 variants, 6.3 MB total; features are `efont-cn-{size}` and `efont-cn-{size}-{b|i|bi}`, plus `efont-cn` (the five regular) and `efont-cn-all` (everything, enabled by default). ~7.5k glyphs (common GB2312 hanzi + ASCII + symbols); missing glyphs fall back to `max_width()` (matching LovyanGFX behavior).

## Relationship to u8g2-fonts

An honest comparison — pick what fits:

| | [u8g2-fonts](https://crates.io/crates/u8g2-fonts) | this + main crate |
|---|---|---|
| Positioning | embedded-graphics text renderer + built-in font catalog (1998 fonts) | decoder and data as separate crates |
| Bring your own font | catalog-locked, no raw-byte entry point | `Font::new(any blob)` |
| Dependencies | depends on embedded-graphics | zero-dep decoder, `#![no_std]`, `forbid(unsafe)` |
| CJK bold / italic | none (one decorative bold family aside) | 5 sizes × bold / italic / bold-italic |
| Font licensing | data is not MIT/Apache (flagged in their README) | same layering: code MIT / data under upstream notices (see NOTICE) |

If you want "pick a font and render" and already use embedded-graphics → use u8g2-fonts, it is mature. If you need your own blobs, subsets, no e-g dependency, or CJK emphasis styles → this side.

## Licensing & provenance

- Crate code: MIT (see [LICENSE](LICENSE)).
- Font data: /efont (Electronic Font Open Laboratory, BSD-style), converted to u8g2 arrays by LovyanGFX (FreeBSD). **The data is not relicensed as MIT by this package.** Both upstream notices ship verbatim in [licenses/](licenses/); your obligations are in [NOTICE](NOTICE) — if you flash these fonts into distributed firmware, ship those two notices along.
- Reproducible regeneration:

  ```sh
  lgyf-gen <your LovyanGFX checkout>/src/lgfx/Fonts/efont/lgfx_efont_cn.c -o fonts --only "lgfx_efont_cn_*"
  sha256sum fonts/*.bin > fonts/SHA256SUMS
  ```

  The output must be byte-identical to this package's `fonts/` and [fonts/SHA256SUMS](fonts/SHA256SUMS).

## Development

```sh
cargo test                 # full regression (default = all fonts)
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
cargo run --example text_metrics
```

CI (`.github/workflows/ci.yml`) runs fmt / clippy in both feature modes / the full test suite / a package-size guard (crates.io cap is 10 MB).

## Pre-publish checklist

- [ ] Confirm `repository` in `Cargo.toml` points at the real GitHub repo
- [ ] `cargo package` size < 10 MB (currently ≈ 5 MB compressed, 6.0 MB unpacked)
- [ ] `cargo publish --dry-run`
- [ ] `cargo publish`

## See also

- [`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts) — the no_std decoder and the `lgyf-gen` extraction tool
- [LovyanGFX](https://github.com/lovyan03/LovyanGFX) — source of the font arrays (u8g2 format)
- [/efont](https://osdn.net/projects/efont/) — the original bitmap fonts

## License

Code MIT © 2026 wangguanghua2099; font data licensing in [NOTICE](NOTICE) and [licenses/](licenses/).
