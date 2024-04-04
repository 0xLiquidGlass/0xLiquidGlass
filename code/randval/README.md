# randval - Generating random values
randval generates a hex value that is secure

## Usage

randval <switch>

### Switches
-c or --clipboard \
(Generate new random value every second to clipboard)

__Note: No switch (i.e. just passing "randval") will only output one value to the terminal at a time__

## Compiling randval

To compile randval, you will need the [rust compiler](https://www.rust-lang.org/tools/install) to be installed

Once the compiler has been installed, head into the `randval/` folder \
and type the following:

`cargo build`

The executable can be found in `target/debug/`
