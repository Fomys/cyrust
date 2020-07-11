use font8x8::UnicodeFonts;
use framebuffer::{Buffer, Color, FbGUI};
use std::error::Error;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut gui = FbGUI::new(Path::new("/dev/fb0"))?;
    println!("OK");
    gui.fill(Color::BLACK);
    println!("OK");
    sleep(Duration::from_secs(1));
    println!("OK");
    gui.update()?;
    print!("B");
    sleep(Duration::from_secs(1));
    gui.fill(Color::WHITE);
    gui.update()?;
    print!("W");
    sleep(Duration::from_secs(1));
    gui.fill(Color::BLACK);
    gui.update()?;
    print!("B");
    sleep(Duration::from_secs(1));
    gui.fill(Color::WHITE);
    gui.update()?;
    print!("W");
    sleep(Duration::from_secs(1));
    gui.write_buf(100, 100, &Buffer::heart().scale(50));
    gui.update()?;
    let mut y = 100;
    let mut x = 10;
    for i in 30..127 {
        gui.write_buf(
            x,
            y,
            &Buffer::from_u8(font8x8::BASIC_FONTS.get(i as u8 as char).unwrap()).scale(5),
        );
        x += 40;
        if x > 1000 {
            y += 100;
            x = 10;
        }
    }
    gui.update()?;
    let mut buffer = Buffer::new((100, 100).into(), Color::BLACK);
    for x in 0..10 {
        buffer.draw_line((50, 50).into(), (10 * x, 0).into(), Color::WHITE);
        buffer.draw_line((50, 50).into(), (10 * x, 99).into(), Color::WHITE);
    }
    for y in 0..10 {
        buffer.draw_line((50, 50).into(), (0, y * 10).into(), Color::WHITE);
        buffer.draw_line((50, 50).into(), (99, y * 10).into(), Color::WHITE);
    }
    gui.write_buf(1300, 100, &buffer.scale(4));
    gui.update()?;
    Ok(())
}
