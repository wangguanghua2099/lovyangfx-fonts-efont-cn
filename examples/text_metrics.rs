//! 主机侧小示例：用捆绑的 14px 常规字体量一段混排文本。
//! 本包是纯数据，解码器经 dev-dependency（lovyangfx-fonts）引入。
//!
//! ```sh
//! cargo run --example text_metrics
//! ```
use lovyangfx_fonts::Font;

fn main() {
    let font = Font::new(lovyangfx_fonts_efont_cn::EFONT_CN_14_BLOB)
        .expect("bundled blob is a valid u8g2 font");

    let text = "你好，世界！Hello, world!";
    println!("text_width({text:?}) = {} px", font.text_width(text));
    println!("fits in 240 px: {}", font.fits(text, 240));

    let mut pixels = 0usize;
    font.for_each_pixel(text, 240, |_, _| pixels += 1);
    println!("{pixels} foreground pixels (clipped to x<240)");

    println!(
        "font: max {}x{} px, ascent {} px, blob {} bytes",
        font.max_width(),
        font.max_height(),
        font.ascent(),
        font.blob().len()
    );
}
