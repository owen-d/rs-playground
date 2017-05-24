mod lib;
extern crate futures;

use std::io;
use futures::future::{Future, BoxFuture};



fn main() {
    let data = vec![0..10];
    let ctrl = lib::Delegator::new(data);


}
