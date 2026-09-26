//! 主机侧小示例：用捆绑的 14px 常规字体量一段混排文本。
//!
//! ```sh
//! cargo run --example text_metrics
//! ```
fn main() {
    let font = lovyangfx_fonts_efont_cn::efont_cn_14();

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
