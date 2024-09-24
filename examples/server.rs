// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate bytes;
extern crate futures;
extern crate http;
extern crate http_body_util;
extern crate hyper;
extern crate hyper_util;
extern crate tokio;
extern crate tokio_stream;

use std::error::Error;

use bytes::Bytes;
use futures::{Future, FutureExt, TryFutureExt, TryStreamExt};
use http_body_util::BodyExt;
use hyper::body::{Body, Incoming};
use hyper::server::conn::http1::Builder;
use hyper::{service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

type BoxFut = Box<
    dyn Future<
            Output = Result<
                Response<Box<dyn Body<Data = Bytes, Error = hyper::Error> + Unpin>>,
                hyper::Error,
            >,
        > + Unpin,
>;

fn index(req: Request<Incoming>) -> BoxFut {
    let body = req.into_body().map_frame(|frame| {
        if let Some(data) = frame.data_ref() {
            print!("{}", String::from_utf8_lossy(data));
        }

        frame
    });

    let res: Response<Box<dyn Body<Data = _, Error = _> + Unpin>> = Response::new(Box::new(body));

    Box::new(futures::future::ready(Ok(res)))
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:9001";
    let sockets = TcpListener::bind(addr).map_ok(TcpListenerStream::new);

    let run = sockets.and_then(|sockets| {
        sockets.try_for_each_concurrent(3, |socket| {
            let conn = Builder::new().serve_connection(TokioIo::new(socket), service_fn(index));

            conn.map(|r| {
                eprintln!("done serving connection: {r:?}");
                Ok(())
            })
        })
    });

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run)?;

    Ok(())
}
