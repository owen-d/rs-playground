mod lib;
extern crate futures;

use std::io;
use futures::future::{Future, BoxFuture};



fn main() {
    let data: Vec<u8> = (0..10).collect();
    let mut ctrl = lib::Delegator::new(data).unwrap();
    let res = ctrl.ship(0).wait();


    println!("{}", res.unwrap());


}
