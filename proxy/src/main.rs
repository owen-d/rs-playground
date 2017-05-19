#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate futures_cpupool;
extern crate serde;
extern crate serde_json;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate hyper;

use std::io;

use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;
use tokio_minihttp::{Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;
use hyper::Client as HttpClient;
use std::sync::Arc;


struct Server {
    client: Arc<HttpClient>,
    thread_pool: CpuPool,
}

#[derive(Serialize)]
struct Message {
    status: u16,
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        let client = self.client.clone();
        let msg = self.thread_pool.spawn_fn(move || {
            let res = client.get("http://www.google.com")
                .send()
                .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("timeout: {}", e))
            })?;

            Ok(Message {
                status: res.status_raw().0
            })
        });

        msg.map(|msg| {
            let json = serde_json::to_string(&msg).unwrap();
            let mut response = Response::new();
            response.body(&json);
            response
        }).boxed()

    }
}




fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let thread_pool = CpuPool::new(10);
    let hyper_client = Arc::new(HttpClient::new());

    TcpServer::new(tokio_minihttp::Http, addr).serve(move || {
        Ok(Server {
            client: hyper_client.clone(),
            thread_pool: thread_pool.clone()
        })
    })
}
