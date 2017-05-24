extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;

use self::futures::{future, Future, Stream, Sink};
use self::future::BoxFuture;
use self::futures::sync::mpsc;
use self::tokio_core::reactor::Core;
use std::{ops, thread, io};

pub struct Delegator<T> {
    data: Option<Vec<Result<T, ()>>>,
}

impl<T> Delegator<T>
    where T: Send + Sync + ops::Add<Output = T> + 'static
{
    pub fn new(data: Vec<T>) -> io::Result<Self> {
        Ok(Delegator {
            data: Some(data.into_iter().map(Ok).collect()),
        })
    }

    pub fn ship(&mut self, initializer: T) -> BoxFuture<T, io::Error> {
        let data = self.data.take().expect("must have data to process");
        let (tx, rx) = mpsc::channel(1);
        let mut core = Core::new().expect("failed to create core");
        let remote = core.remote();

        let _stream = futures::stream::iter(data).map(move |x| {
            let tx = tx.clone();
            let remote = remote.clone();
            thread::spawn(move || {

                remote.spawn(|_| {
                    let res: Result<T, ()> = Ok(x);
                    tx
                        .send(res)
                        .then(|tx| {
                            match tx {
                                Ok(_tx) => {
                                    println!("Sink flushed");
                                    Ok(())
                                }
                                Err(e) => {
                                    println!("Sink failed! {:?}", e);
                                    Err(())
                                }
                            }
                        })
                })
            })
        });


        let res = rx
            .fold(initializer, |a, b| {
                Ok(a + b.unwrap())
            })
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failure running thread jobs"));

        future::result(core.run(res)).boxed()

        // let res = rx
        //     .collect()
        //     .wait()
        //     .map(|completed| completed.into_iter().fold(initializer, |a, b| a + b))
        //     .map_err(|_| io::Error::new(io::ErrorKind::Other, "failure running thread jobs"));
        // return future::result(res).boxed();
    }
}
