#[macro_use]
extern crate afl;
extern crate rsass;

use rsass::compile_scss;

fn main() {
    fuzz!(|data: &[u8]| {
        let _ = compile_scss(data, Default::default())
                    .map(|s| String::from_utf8(s).unwrap());
    });
}