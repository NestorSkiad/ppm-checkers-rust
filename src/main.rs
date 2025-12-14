use std::fs::File;
use std::io::Write;

static FRAME_COUNT: usize = 10;
static WIDTH_MUL: usize = 16;
static HEIGHT_MUL: usize = 9;
static RESOLUTION_MULTIPLIER: usize = 60;

fn main() -> std::io::Result<()> {
    let width = WIDTH_MUL * RESOLUTION_MULTIPLIER;
    let height = HEIGHT_MUL * RESOLUTION_MULTIPLIER;

    for f in 0..FRAME_COUNT {
        let mut bytes: Vec<u8> = Vec::with_capacity(15 + (width * height * 3));
        println!("starting capacity: {}", bytes.capacity());

        bytes.extend_from_slice(b"P6\n");
        bytes.extend_from_slice(format!("{} {}\n", width, height).as_bytes());
        bytes.extend_from_slice(format!("{}\n", u8::MAX).as_bytes());

        for y in 0..height {
            for x in 0..width {
                if ((x + f) / RESOLUTION_MULTIPLIER + (y + f) / RESOLUTION_MULTIPLIER) % 2 == 0 {
                    bytes.extend_from_slice(&[0xFF, 0x00, 0x00]);
                } else {
                    bytes.extend_from_slice(&[0x00, 0x00, 0x00]);
                }
            }
        }

        println!("items: {}, capacity: {}", bytes.len(), bytes.capacity());

        let file_path = format!("output/{:04}.ppm", f);

        let mut image_file = File::create(file_path)?;
        image_file.write_all(bytes.as_slice())?;
    }
    Ok(())
}
