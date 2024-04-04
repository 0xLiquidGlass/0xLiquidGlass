// Documentation:
// https://doc.rust-lang.org/std/time/struct.SystemTime.html#

use std::time::{ SystemTime, UNIX_EPOCH };

fn main() {
    let timestamp = SystemTime::now();
    match timestamp.duration_since(UNIX_EPOCH) {
        Ok(unix_time) => println!("{}", unix_time.as_secs()),
        Err(epoch_error) => panic!("{:?}", epoch_error),
    }
}
