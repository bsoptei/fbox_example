extern crate im;

use im::vector::Vector;

use std::env;
use std::fs::read as read_from_path;
use std::io::Error;

pub mod fbox {

    pub struct FBox<FIn, FOut> {
        f: Box<Fn(FIn) -> FOut>
    }

    impl<FIn: 'static, FOut: 'static> FBox<FIn, FOut> {

        pub fn new(f: impl Fn(FIn) -> FOut + 'static) -> FBox<FIn, FOut> {
            FBox { f: Box::new(f) }
        }

        pub fn apply(&self, a: FIn) -> FOut {
            (self.f)(a)
        }

        pub fn compose<GIn: 'static>(self, g: impl Fn(GIn) -> FIn + 'static) -> FBox<GIn, FOut> {
            FBox::new(move |x| (self.f)(g(x)))
        }

        pub fn and_then<GOut: 'static>(self, g: impl Fn(FOut) -> GOut + 'static) -> FBox<FIn, GOut> {
            FBox::new(move |x| g((self.f)(x)))
        }

    }

}

fn main() {
    use fbox::FBox;

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
