use honggfuzz::fuzz;
use tree_buf::prelude::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let data_str: String = std::str::from_utf8(data).unwrap().to_string();
            let _ = encode(&data_str);
        });
    }
}
