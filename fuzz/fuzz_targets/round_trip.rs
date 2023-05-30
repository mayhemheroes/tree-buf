use honggfuzz::fuzz;
use tree_buf::prelude::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let data_str: String = std::str::from_utf8(data).unwrap().to_string();
            let bytes = encode(&data_str);
            let copy: String = decode(&bytes).unwrap();
            assert_eq!(&copy, &data_str);
        });
    }
}
