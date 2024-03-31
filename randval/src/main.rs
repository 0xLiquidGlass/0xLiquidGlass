use rand::prelude::*;
use hex;

fn main(){
    let mut rng = rand::thread_rng();
    let mut buffer = [0u8; 8];

    rng.fill(&mut buffer);
    let array_bytes = &buffer;
    let sliced_bytes = &array_bytes.as_slice();
    let hexed_bytes = hex::encode(&sliced_bytes);
    println!("{:?}", &hexed_bytes);
}
