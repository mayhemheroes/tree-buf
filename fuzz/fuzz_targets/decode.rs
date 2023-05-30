use honggfuzz::fuzz;
use tree_buf::prelude::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let data_vec = data.to_vec();
            let _ = decode::<String>(&data_vec);
        });
    }
}
