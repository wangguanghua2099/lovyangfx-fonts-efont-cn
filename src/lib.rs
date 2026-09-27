//! Bundled [eFont](https://osdn.net/projects/efont/) CN bitmap fonts — **pure
//! data**: 20 raw u8g2 wire-format blobs (five sizes 10 / 12 / 14 / 16 / 24 px,
//! each in regular / bold / italic / bold-italic), extracted from LovyanGFX'
//! C sources with `lgyf-gen` and byte-identical to the upstream arrays.
//!
//! This crate has **zero dependencies** and ships no decoder. Decode with
//! [`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts):
//!
//! ```
//! use lovyangfx_fonts::Font;
//! use lovyangfx_fonts_efont_cn::EFONT_CN_14_BLOB;
//!
//! let font = Font::new(EFONT_CN_14_BLOB).unwrap();
//! assert!(font.has_glyph('你'));
//! assert!(font.text_width("你好AI") > 0);
//! ```
//!
//! or take the one-line route through the main crate's `fonts` module
//! (lovyangfx-fonts 0.2+):
//!
//! ```toml
//! lovyangfx-fonts = { version = "0.2", features = ["efont-cn-14"] }
//! ```
//!
//! ```text
//! let font = lovyangfx_fonts::fonts::efont_cn_14();   // zero-copy Font handle
//! ```
//!
//! Every blob sits behind a cargo feature named after the font (`efont-cn-14`,
//! `efont-cn-14-b`, …). **All blobs are enabled by default**; pick a subset with
//! `default-features = false` plus the features you need — disabled fonts are
//! not linked into the binary.
//!
//! The fonts cover ~7.5k glyphs (GB2312 常用汉字 + ASCII + 符号). The bold and
//! italic styles are the distinctive part: u8g2's own CJK catalog has neither.
//!
//! # Why pure data?
//!
//! `lovyangfx-fonts` 0.2 bridges this crate as an *optional dependency*. A
//! runtime dependency in the other direction (this crate returning `Font`
//! handles) would either form a package cycle or drag a second copy of the
//! decoder into every build with an incompatible `Font` type. Data-only keeps
//! the graph a tree and this crate's identity clean: data plus the upstream
//! notices that must travel with it.
//!
//! # Licensing
//!
//! Crate code is MIT. The **font data** is derived from /efont via LovyanGFX
//! and stays under its upstream notices — see `NOTICE` and `licenses/`,
//! shipped with this package. Do not relicense the data as MIT.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// 为一个字体变体生成 `*_BLOB` 常量，挂在对应 feature 下。
macro_rules! bundled_blob {
    ($feature:literal, $const_name:ident, $file:literal, $size:literal, $style:literal) => {
        #[cfg(feature = $feature)]
        #[doc = concat!("Raw u8g2 wire-format blob of the ", $size, " px **", $style,
                    "** font (`", $file, "`), byte-identical to the LovyanGFX array it was ",
                    "extracted from. Requires feature `", $feature, "`.")]
        pub const $const_name: &[u8] = include_bytes!(concat!("../fonts/", $file));
    };
}

bundled_blob!(
    "efont-cn-10",
    EFONT_CN_10_BLOB,
    "efont_cn_10.bin",
    "10",
    "regular"
);
bundled_blob!(
    "efont-cn-10-b",
    EFONT_CN_10_B_BLOB,
    "efont_cn_10_b.bin",
    "10",
    "bold"
);
bundled_blob!(
    "efont-cn-10-bi",
    EFONT_CN_10_BI_BLOB,
    "efont_cn_10_bi.bin",
    "10",
    "bold-italic"
);
bundled_blob!(
    "efont-cn-10-i",
    EFONT_CN_10_I_BLOB,
    "efont_cn_10_i.bin",
    "10",
    "italic"
);

bundled_blob!(
    "efont-cn-12",
    EFONT_CN_12_BLOB,
    "efont_cn_12.bin",
    "12",
    "regular"
);
bundled_blob!(
    "efont-cn-12-b",
    EFONT_CN_12_B_BLOB,
    "efont_cn_12_b.bin",
    "12",
    "bold"
);
bundled_blob!(
    "efont-cn-12-bi",
    EFONT_CN_12_BI_BLOB,
    "efont_cn_12_bi.bin",
    "12",
    "bold-italic"
);
bundled_blob!(
    "efont-cn-12-i",
    EFONT_CN_12_I_BLOB,
    "efont_cn_12_i.bin",
    "12",
    "italic"
);

bundled_blob!(
    "efont-cn-14",
    EFONT_CN_14_BLOB,
    "efont_cn_14.bin",
    "14",
    "regular"
);
bundled_blob!(
    "efont-cn-14-b",
    EFONT_CN_14_B_BLOB,
    "efont_cn_14_b.bin",
    "14",
    "bold"
);
bundled_blob!(
    "efont-cn-14-bi",
    EFONT_CN_14_BI_BLOB,
    "efont_cn_14_bi.bin",
    "14",
    "bold-italic"
);
bundled_blob!(
    "efont-cn-14-i",
    EFONT_CN_14_I_BLOB,
    "efont_cn_14_i.bin",
    "14",
    "italic"
);

bundled_blob!(
    "efont-cn-16",
    EFONT_CN_16_BLOB,
    "efont_cn_16.bin",
    "16",
    "regular"
);
bundled_blob!(
    "efont-cn-16-b",
    EFONT_CN_16_B_BLOB,
    "efont_cn_16_b.bin",
    "16",
    "bold"
);
bundled_blob!(
    "efont-cn-16-bi",
    EFONT_CN_16_BI_BLOB,
    "efont_cn_16_bi.bin",
    "16",
    "bold-italic"
);
bundled_blob!(
    "efont-cn-16-i",
    EFONT_CN_16_I_BLOB,
    "efont_cn_16_i.bin",
    "16",
    "italic"
);

bundled_blob!(
    "efont-cn-24",
    EFONT_CN_24_BLOB,
    "efont_cn_24.bin",
    "24",
    "regular"
);
bundled_blob!(
    "efont-cn-24-b",
    EFONT_CN_24_B_BLOB,
    "efont_cn_24_b.bin",
    "24",
    "bold"
);
bundled_blob!(
    "efont-cn-24-bi",
    EFONT_CN_24_BI_BLOB,
    "efont_cn_24_bi.bin",
    "24",
    "bold-italic"
);
bundled_blob!(
    "efont-cn-24-i",
    EFONT_CN_24_I_BLOB,
    "efont_cn_24_i.bin",
    "24",
    "italic"
);
