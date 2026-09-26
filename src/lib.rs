//! Bundled [eFont](https://osdn.net/projects/efont/) CN bitmap fonts for the
//! [`lovyangfx-fonts`](https://crates.io/crates/lovyangfx-fonts) decoder —
//! five sizes (10 / 12 / 14 / 16 / 24 px), each in four styles (regular,
//! bold, italic, bold-italic), u8g2 wire format, extracted from LovyanGFX'
//! C sources with `lgyf-gen`.
//!
//! The decoder lives in [`lovyangfx_fonts`]; this crate only carries the data
//! and hands out zero-copy [`Font`](lovyangfx_fonts::Font) handles:
//!
//! ```
//! use lovyangfx_fonts_efont_cn::efont_cn_14;
//!
//! let font = efont_cn_14();
//! assert!(font.has_glyph('你'));
//! assert!(font.text_width("你好AI") > 0);
//! ```
//!
//! Every accessor sits behind a cargo feature named after the font
//! (`efont-cn-14`, `efont-cn-14-b`, …). **All fonts are enabled by default**;
//! pick a subset with `default-features = false` plus the features you need —
//! disabled fonts are not linked into the binary.
//!
//! The fonts cover ~7.5k glyphs (GB2312 常用汉字 + ASCII + 符号). The bold and
//! italic styles are the distinctive part: u8g2's own CJK catalog has neither.
//!
//! # Licensing
//!
//! Crate code is MIT. The **font data** is derived from /efont via LovyanGFX
//! and stays under its upstream notices — see `NOTICE` and `licenses/`,
//! shipped with this package. Do not relicense the data as MIT.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

// 不在模块级 `use lovyangfx_fonts::Font`：`--no-default-features` 时全部条目
// 都被 cfg 掉，顶层导入会触发 unused_imports。宏内直接写全路径。
/// 为一个字体变体生成 `*_BLOB` 常量与同名蛇形访问函数，均挂在对应 feature 下。
macro_rules! bundled_font {
    ($feature:literal, $const_name:ident, $fn_name:ident, $file:literal, $size:literal, $style:literal) => {
        #[cfg(feature = $feature)]
        #[doc = concat!("Raw u8g2 wire-format blob of the ", $size, " px **", $style,
                    "** font (`", $file, "`), byte-identical to the LovyanGFX array it was ",
                    "extracted from. Requires feature `", $feature, "`.")]
        pub const $const_name: &[u8] = include_bytes!(concat!("../fonts/", $file));

        #[cfg(feature = $feature)]
        #[doc = concat!("Zero-copy [`Font`](lovyangfx_fonts::Font) handle over the bundled ",
                    $size, " px **", $style,
                    "** font. Requires feature `", $feature,
                    "`. Blob validity is asserted by the crate's own test suite, so the only ",
                    "failure mode (a blob shorter than a header) cannot occur here.")]
        pub fn $fn_name() -> lovyangfx_fonts::Font<'static> {
            lovyangfx_fonts::Font::new($const_name)
                .expect("bundled blob is a valid u8g2 font (covered by tests)")
        }
    };
}

bundled_font!(
    "efont-cn-10",
    EFONT_CN_10_BLOB,
    efont_cn_10,
    "efont_cn_10.bin",
    "10",
    "regular"
);
bundled_font!(
    "efont-cn-10-b",
    EFONT_CN_10_B_BLOB,
    efont_cn_10_b,
    "efont_cn_10_b.bin",
    "10",
    "bold"
);
bundled_font!(
    "efont-cn-10-bi",
    EFONT_CN_10_BI_BLOB,
    efont_cn_10_bi,
    "efont_cn_10_bi.bin",
    "10",
    "bold-italic"
);
bundled_font!(
    "efont-cn-10-i",
    EFONT_CN_10_I_BLOB,
    efont_cn_10_i,
    "efont_cn_10_i.bin",
    "10",
    "italic"
);

bundled_font!(
    "efont-cn-12",
    EFONT_CN_12_BLOB,
    efont_cn_12,
    "efont_cn_12.bin",
    "12",
    "regular"
);
bundled_font!(
    "efont-cn-12-b",
    EFONT_CN_12_B_BLOB,
    efont_cn_12_b,
    "efont_cn_12_b.bin",
    "12",
    "bold"
);
bundled_font!(
    "efont-cn-12-bi",
    EFONT_CN_12_BI_BLOB,
    efont_cn_12_bi,
    "efont_cn_12_bi.bin",
    "12",
    "bold-italic"
);
bundled_font!(
    "efont-cn-12-i",
    EFONT_CN_12_I_BLOB,
    efont_cn_12_i,
    "efont_cn_12_i.bin",
    "12",
    "italic"
);

bundled_font!(
    "efont-cn-14",
    EFONT_CN_14_BLOB,
    efont_cn_14,
    "efont_cn_14.bin",
    "14",
    "regular"
);
bundled_font!(
    "efont-cn-14-b",
    EFONT_CN_14_B_BLOB,
    efont_cn_14_b,
    "efont_cn_14_b.bin",
    "14",
    "bold"
);
bundled_font!(
    "efont-cn-14-bi",
    EFONT_CN_14_BI_BLOB,
    efont_cn_14_bi,
    "efont_cn_14_bi.bin",
    "14",
    "bold-italic"
);
bundled_font!(
    "efont-cn-14-i",
    EFONT_CN_14_I_BLOB,
    efont_cn_14_i,
    "efont_cn_14_i.bin",
    "14",
    "italic"
);

bundled_font!(
    "efont-cn-16",
    EFONT_CN_16_BLOB,
    efont_cn_16,
    "efont_cn_16.bin",
    "16",
    "regular"
);
bundled_font!(
    "efont-cn-16-b",
    EFONT_CN_16_B_BLOB,
    efont_cn_16_b,
    "efont_cn_16_b.bin",
    "16",
    "bold"
);
bundled_font!(
    "efont-cn-16-bi",
    EFONT_CN_16_BI_BLOB,
    efont_cn_16_bi,
    "efont_cn_16_bi.bin",
    "16",
    "bold-italic"
);
bundled_font!(
    "efont-cn-16-i",
    EFONT_CN_16_I_BLOB,
    efont_cn_16_i,
    "efont_cn_16_i.bin",
    "16",
    "italic"
);

bundled_font!(
    "efont-cn-24",
    EFONT_CN_24_BLOB,
    efont_cn_24,
    "efont_cn_24.bin",
    "24",
    "regular"
);
bundled_font!(
    "efont-cn-24-b",
    EFONT_CN_24_B_BLOB,
    efont_cn_24_b,
    "efont_cn_24_b.bin",
    "24",
    "bold"
);
bundled_font!(
    "efont-cn-24-bi",
    EFONT_CN_24_BI_BLOB,
    efont_cn_24_bi,
    "efont_cn_24_bi.bin",
    "24",
    "bold-italic"
);
bundled_font!(
    "efont-cn-24-i",
    EFONT_CN_24_I_BLOB,
    efont_cn_24_i,
    "efont_cn_24_i.bin",
    "24",
    "italic"
);
