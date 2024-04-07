// Documentation
// Read file with buffer: https://doc.rust-lang.org/std/fs/struct.File.html
// qrcode: https://crates.io/crates/qrcode

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use qrcode::QrCode;
use qrcode::render::unicode;

fn use_file_data(filename: &str) -> std::io::Result<()> {
    let file = File::open(&filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let _ = generate_qrcode(&mut contents);
    Ok(())
}

fn generate_qrcode(input_data: &str) {
    let toqr = QrCode::new(&input_data).unwrap();
    let genqr = toqr.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", genqr);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    if &args[1] == "-v" || &args[1] == "--version" {
        println!("v0.1.0");
        return;
    } else if &args[1] == "-f" || &args[1] == "--file" {
        let filename = &args[2].trim_start();
        if *filename != "" {
            let _ = use_file_data(&filename);
            return;
        } else {
            println!("Please input filename");
            return;
        }
    } else {
        let input_string = &args[1];
        let _ = generate_qrcode(&input_string);
        return;
    }
}
