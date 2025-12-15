use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;

static FRAME_COUNT: usize = 10_000;
static WIDTH_MUL: usize = 16;
static HEIGHT_MUL: usize = 9;
static RESOLUTION_MULTIPLIER: usize = 60;

fn process_frame(f: usize, width: &usize, height: &usize) -> () {
    let mut bytes: Vec<u8> = Vec::with_capacity(15 + (width * height * 3));
    let start_capacity = bytes.capacity();
    // println!("starting capacity: {}", bytes.capacity());

    bytes.extend_from_slice(b"P6\n");
    bytes.extend_from_slice(format!("{} {}\n", width, height).as_bytes());
    bytes.extend_from_slice(format!("{}\n", u8::MAX).as_bytes());

    for y in 0..*height {
        for x in 0..*width {
            if ((x + f) / RESOLUTION_MULTIPLIER + (y + f) / RESOLUTION_MULTIPLIER) % 2 == 0 {
                bytes.extend_from_slice(&[0xFF, 0x00, 0x00]);
            } else {
                bytes.extend_from_slice(&[0x00, 0x00, 0x00]);
            }
        }
    }

    // println!("items: {}, capacity: {}", bytes.len(), bytes.capacity());
    assert_eq!(start_capacity, bytes.capacity());
    assert_eq!(bytes.capacity(), bytes.len());

    let file_path = format!("output/{:04}.ppm", f);

    let mut image_file = File::create(file_path).expect("couldn't create file");
    image_file
        .write_all(bytes.as_slice())
        .expect("couldn't write to file");
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let range = 0..FRAME_COUNT;

    let width = WIDTH_MUL * RESOLUTION_MULTIPLIER;
    let height = HEIGHT_MUL * RESOLUTION_MULTIPLIER;

    let _: () = if args[1] == "s" {
        println!("executing single threaded");

        range
            .into_iter()
            .map(|f| {
                process_frame(f, &width, &height);
            })
            .collect()
    } else {
        println!("executing multi threaded");

        range
            .into_par_iter()
            .map(|f| {
                process_frame(f, &width, &height);
            })
            .collect()
    };

    Ok(())
}
