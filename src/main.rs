fn main() {
    let width = 600;
    let height = 600;

    let mut pixel_buffer = image::ImageBuffer::new(width, height);

    for (_x, _y, pixel) in pixel_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([208 as u8, 185 as u8, 240 as u8]);
    }

    pixel_buffer.save("out.png").unwrap();

    println!("Generated image!");
}
