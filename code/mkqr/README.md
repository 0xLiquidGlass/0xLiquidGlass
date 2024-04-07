## mkqr - generate QR code
mkqr allows you to generate a QR code based on a string passed from the terminal or \
a file that contains a text

## Usage

`mkqr <switch>`

`mkqr -f <filename>`

### Switches

-v or --version \
(See the version for mkqr)

-f or --file \
(Generate a QR code from a file that contains text)

__Note: By default, running `mkqr` without switches will generate a QR code \
based on the string passed on the terminal__

## Compiling mkqr

To compile mkqr:

- Install the [rust compiler](https://www.rust-lang.org/tools/install) if it has not yet been installed
- Go to the `mkqr` folder and run the command `cargo build`
- The executable can be found in `target/debug` folder
