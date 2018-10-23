extern crate fbox;
extern crate im;

use fbox::*;
use im::vector::Vector;

use std::env;
use std::fs::read as read_from_path;
use std::io::Error;

fn main() {

    type Res = Result<Vec<u8>, Error>;

    let args = || env::args().collect::<Vec<String>>();

    fn extract_bytes(bytes_in_a_result: Res) -> Vec<u8> {
        bytes_in_a_result.unwrap_or_else(
            |error| {
                println!("{:?}", error); vec![]
            }
        )
    }

    let bytes_from_path =
        FBox::new(read_from_path)
            .and_then(extract_bytes)
            .and_then(Vector::from);

    let res =
        args().get(1).map(
            |path| bytes_from_path.apply(
                path.to_string()
            )
        );

    println!("{:?}", res);
}
