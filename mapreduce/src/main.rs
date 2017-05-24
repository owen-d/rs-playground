mod lib;
extern crate futures;

use futures::future::Future;



fn main() {
    let data: Vec<u64> = (0..100000).collect();
    let mut ctrl = lib::Delegator::new(data).unwrap();
    let res = ctrl.ship(0).wait();


    println!("{}", res.unwrap());


}
