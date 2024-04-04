// Documentation
// args: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
// sleep: https://doc.rust-lang.org/std/thread/fn.sleep.html
// Clipboard: https://docs.rs/cli-clipboard/latest/cli_clipboard/
// rand: https://docs.rs/rand/latest/rand/
// hex: https://docs.rs/hex/latest/hex/

use std::env;
use std:: { thread, time };
use cli_clipboard::{ ClipboardContext, ClipboardProvider };
use rand::prelude::*;
use hex;

fn get_randval() -> String {
    let mut rng = rand::thread_rng();
    let mut buffer = [0u8; 8];

    rng.fill(&mut buffer);
    let array_bytes = &buffer;
    let sliced_bytes = &array_bytes.as_slice();
    let hexed_bytes = hex::encode(&sliced_bytes);
    println!("{:?}", &hexed_bytes);

    return hexed_bytes;
}

fn set_clipboard() {
    let sleep_duration = time::Duration::from_secs(1);

    let mut ctx = ClipboardContext::new().unwrap();

    let clipboard_data = get_randval();

    ctx.set_contents(clipboard_data.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), clipboard_data);

    thread::sleep(sleep_duration);

    let _ = ctx.clear();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        get_randval();
        return;
    }

    let first_switch = &args[1];

    if first_switch == "-c" || first_switch == "--clipboard" {
        loop {
            set_clipboard();
        }    
    }
}