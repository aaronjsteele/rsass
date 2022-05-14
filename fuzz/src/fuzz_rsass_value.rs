#[macro_use]
extern crate afl;
extern crate rsass;

use afl::fuzz;
use rsass::compile_value;

fn main() {
    fuzz!(|data: &[u8]| {
        let _ = compile_value(data);
    });
}