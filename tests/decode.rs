//! 捆绑数据的全量回归测试。
//!
//! 主 crate（lovyangfx-fonts）不带数据，真库用例要靠 `LGYF_FONT` 环境变量指路；
//! 本 crate 的 blob 就在包里，所以这套测试**默认全跑**——这正是"数据随包分发"
//! 换来的第一红利：任何人 `cargo test` 都能复现主 crate 只能在作者机器上跑的
//! 那套真库回归。
//!
//! 每个启用的字体变体跑同一套 `suite`：
//!
//! 1. 金标准字节数（防文件换错 / 截断）；
//! 2. 头部合理性（max_width / max_height / ascent）；
//! 3. ASCII + 常用汉字覆盖；
//! 4. 度量一致性 + 基线对齐；
//! 5. ~23.9k 码位逐个解码：不 panic、像素数不失控；
//! 6. 行宽截断。
#![allow(dead_code)]

use lovyangfx_fonts::Font;
#[allow(unused_imports)] // --no-default-features 时没有任何 per_font 展开，导入空悬
use lovyangfx_fonts_efont_cn as fonts;

/// 参与扫描的码位段：ASCII、Latin-1 尾、常用符号、假名/标点、CJK、全角。
fn ranges() -> Vec<(u32, u32)> {
    vec![
        (0x20, 0x7F),
        (0xA0, 0x800),
        (0x2000, 0x2200),
        (0x3000, 0x3100),
        (0x4E00, 0x9FAF),
        (0xFF00, 0xFFF0),
    ]
}

fn suite(font: Font<'static>, name: &str, expected_bytes: usize) {
    // 1. 金标准：blob 字节数必须与随包发布时一致
    assert_eq!(
        font.blob().len(),
        expected_bytes,
        "{name}: blob 大小变了（{} != {expected_bytes}），文件可能被换错或截断",
        font.blob().len()
    );

    // 2. 头部合理性
    assert!(
        font.max_width() > 0 && font.max_width() < 64,
        "{name}: max_width={}",
        font.max_width()
    );
    assert!(
        font.max_height() > 0 && font.max_height() < 64,
        "{name}: max_height={}",
        font.max_height()
    );
    assert!(
        font.ascent() > 0 && font.ascent() <= font.max_height() + 8,
        "{name}: ascent={}",
        font.ascent()
    );

    // 3. ASCII + 常用汉字覆盖
    for c in (0x20u8..0x7F).map(char::from) {
        assert!(font.has_glyph(c), "{name}: ASCII {c:?} 缺字形");
    }
    for c in "你好吗？我的是了0123456789:-%".chars() {
        assert!(font.has_glyph(c), "{name}: {c:?} 缺字形");
    }

    // 4. 度量一致性 + 基线对齐（同一行所有像素落在合理竖直范围内）
    let a = font.glyph('a').unwrap().metric();
    assert_eq!(font.text_width("aaa"), a.xadvance * 3, "{name}");
    assert!(font.fits("aaa", a.xadvance * 3), "{name}");
    assert!(!font.fits("aaa", a.xadvance * 3 - 1), "{name}");
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    font.for_each_pixel("Hya你", 240, |_, y| {
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    });
    assert!(
        min_y >= 0 && max_y < font.max_height() + 8,
        "{name}: y=[{min_y},{max_y}]"
    );

    // 5. 逐码位解码：不许 panic（解码器自身保证）、像素数不许失控（游程解飞的征兆）
    let cap = (font.max_width() * (font.max_height() + 8)) as usize;
    let mut checked = 0usize;
    let mut drawn = 0usize;
    for (lo, hi) in ranges() {
        for cp in lo..=hi {
            let Some(c) = char::from_u32(cp) else {
                continue;
            };
            checked += 1;
            let s = c.to_string();
            let mut px = 0usize;
            font.for_each_pixel(&s, 240, |_, _| px += 1);
            assert!(
                px <= cap * 4,
                "{name}: U+{cp:04X} 像素 {px} 超出合理范围（cap {cap}）"
            );
            if px > 0 {
                drawn += 1;
            }
        }
    }
    assert!(
        checked > 20_000,
        "{name}: 扫描范围异常，只查了 {checked} 个码位"
    );
    assert!(
        drawn > 5_000,
        "{name}: 只有 {drawn} 个码位有像素，blob 可能不对"
    );

    // 6. 行宽截断
    let long = "这是一段很长很长的中文文本用来验证行宽截断行为是否生效确实很长";
    let limit = 240;
    let mut over = 0usize;
    font.for_each_pixel(long, limit, |x, _| {
        if x >= limit {
            over += 1;
        }
    });
    assert_eq!(over, 0, "{name}: 有像素画到了行宽之外");
    assert!(
        font.text_width(long) > limit,
        "{name}: 行宽应超限才会触发截断"
    );
}

/// 为每个启用的字体变体生成一个测试；未启用的 feature 展开为空。
/// 0.2 起本包是纯数据：测试通过 dev-dependency 引解码器，自己建句柄。
macro_rules! per_font {
    ($($(#[$cfg:meta])* $test:ident => $blob:ident, $bytes:literal;)*) => {
        $(
            $(#[$cfg])*
            #[test]
            fn $test() {
                suite(
                    Font::new(fonts::$blob).expect("bundled blob is a valid u8g2 font"),
                    stringify!($blob),
                    $bytes,
                );
            }
        )*
    };
}

per_font! {
    #[cfg(feature = "efont-cn-10")]    efont_cn_10    => EFONT_CN_10_BLOB,    158417;
    #[cfg(feature = "efont-cn-10-b")]  efont_cn_10_b  => EFONT_CN_10_B_BLOB,  159551;
    #[cfg(feature = "efont-cn-10-bi")] efont_cn_10_bi => EFONT_CN_10_BI_BLOB, 174904;
    #[cfg(feature = "efont-cn-10-i")]  efont_cn_10_i  => EFONT_CN_10_I_BLOB,  170008;

    #[cfg(feature = "efont-cn-12")]    efont_cn_12    => EFONT_CN_12_BLOB,    213444;
    #[cfg(feature = "efont-cn-12-b")]  efont_cn_12_b  => EFONT_CN_12_B_BLOB,  211952;
    #[cfg(feature = "efont-cn-12-bi")] efont_cn_12_bi => EFONT_CN_12_BI_BLOB, 235895;
    #[cfg(feature = "efont-cn-12-i")]  efont_cn_12_i  => EFONT_CN_12_I_BLOB,  232931;

    #[cfg(feature = "efont-cn-14")]    efont_cn_14    => EFONT_CN_14_BLOB,    262233;
    #[cfg(feature = "efont-cn-14-b")]  efont_cn_14_b  => EFONT_CN_14_B_BLOB,  267590;
    #[cfg(feature = "efont-cn-14-bi")] efont_cn_14_bi => EFONT_CN_14_BI_BLOB, 293038;
    #[cfg(feature = "efont-cn-14-i")]  efont_cn_14_i  => EFONT_CN_14_I_BLOB,  288018;

    #[cfg(feature = "efont-cn-16")]    efont_cn_16    => EFONT_CN_16_BLOB,    318199;
    #[cfg(feature = "efont-cn-16-b")]  efont_cn_16_b  => EFONT_CN_16_B_BLOB,  320446;
    #[cfg(feature = "efont-cn-16-bi")] efont_cn_16_bi => EFONT_CN_16_BI_BLOB, 357031;
    #[cfg(feature = "efont-cn-16-i")]  efont_cn_16_i  => EFONT_CN_16_I_BLOB,  346363;

    #[cfg(feature = "efont-cn-24")]    efont_cn_24    => EFONT_CN_24_BLOB,    550804;
    #[cfg(feature = "efont-cn-24-b")]  efont_cn_24_b  => EFONT_CN_24_B_BLOB,  564226;
    #[cfg(feature = "efont-cn-24-bi")] efont_cn_24_bi => EFONT_CN_24_BI_BLOB, 601696;
    #[cfg(feature = "efont-cn-24-i")]  efont_cn_24_i  => EFONT_CN_24_I_BLOB,  576487;
}
