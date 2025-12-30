use std::env;
mod x;
use crate::x::*;

fn main() {
    let mut args = env::args().skip(1);
    let mut path = args.next();
    if path == Some("-d".to_string()) {
        path = args.next();
        destruction(path);
    } else {
        normal(path);
    }
}
