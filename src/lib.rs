// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578) for
//! hyper.
//!
//! Currently, only the client-side is implemented.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! hyper-multipart-rfc7578 = "0.1.0-alpha3"
//! ```
//!
//! Because the name of this library is really wordy, I recommend shortening it:
//!
//! ```rust
//! use hyper_multipart_rfc7578 as hyper_multipart;
//! ```
//!
//! Using this requires a hyper client compatible with the `multipart::Body`
//! data structure (see the documentation for more detailed examples):
//!
//! ```rust
//! use futures::{Future, FutureExt, TryFutureExt};
//! use hyper::{Method, Request, Uri};
//! use hyper_multipart_rfc7578::client::{self, multipart};
//! use hyper_util::client::legacy::Client;
//! use hyper_util::rt::TokioIo;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let uri = Uri::from_static("http://localhost/upload");
//! let connector = tower::service_fn(|req: Uri| {
//!     let host = req.host().unwrap().to_owned();
//!     let port = req.port_u16().unwrap_or(80);
//!     Box::pin(tokio::net::TcpStream::connect((host, port)).map(|r| r.map(TokioIo::new)))
//! });
//!
//! let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
//! let mut req_builder = Request::get(uri);
//! let mut form = multipart::Form::default();
//!
//! form.add_text("test", "Hello World");
//! let req = form.set_body(req_builder)?;
//!
//! let _ = client.request(req).await;
//! # Ok(())
//! # }
//! ```
mod client_;
mod error;

pub mod client {
    pub use crate::error::Error;

    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    pub mod multipart {
        pub use crate::client_::{Body, BoundaryGenerator, Form, Part};
    }
}
