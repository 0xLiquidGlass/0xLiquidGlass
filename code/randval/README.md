# randval - Generating random values
randval generates a hex value that is secure

## Changing the length
The default value is set to 8 elements long. 

To change the length, this line in `src/main.rs` must be changed:

`let mut buffer = [0u8; 8];`

The number that is in the array can be changed to suit your needs

## Compiling randval

To compile randval, you will need the [rust compiler](https://www.rust-lang.org/tools/install) to be installed

Once the compiler has been installed, head into the `randval/` folder \
and type the following:

`cargo build`

The executable can be found in `target/debug/`
