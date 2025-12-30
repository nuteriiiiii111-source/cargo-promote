use std::env;
mod x;
use crate::x::*;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let mut path = args.next();
    if path == Some("-d".to_string()) {
        path = args.next();
        destruction(path);
    } else {
        normal(path);
    }
}
