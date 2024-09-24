// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::error::Error;

use futures::{FutureExt, TryStreamExt};
use http_body_util::BodyExt;
use hyper::http::Uri;
use hyper::Request;
use hyper_multipart_rfc7578::client::multipart;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = hyper::Uri::from_static("http://127.0.0.1:9001");

    let connector = tower::service_fn(|req: Uri| {
        let host = req.host().unwrap().to_owned();
        let port = req.port_u16().unwrap_or(80);
        Box::pin(TcpStream::connect((host, port)).map(|r| r.map(TokioIo::new)))
    });

    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);

    println!("note: this must be run in the root of the project repository");
    println!("note: run this with the example server running");
    println!("connecting to {}...", uri);

    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    let req_builder = Request::post(uri);

    let req = form.set_body(req_builder).unwrap();

    let mut res = client.request(req).await?.into_data_stream();

    // Read the whole response body.
    while let Some(_frame) = res.try_next().await? {}

    Ok(())
}
