use bytes::{Buf, Bytes};
use futures::stream::{self, Once, Stream};
use http;
use serde;

use std::io::Cursor;

/// Types can be returned as responses to HTTP requests.
pub trait IntoResponse {
    /// Data chunk type
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: Stream<Item = Self::Buf, Error = ::Error>;

    /// Convert the value into a response future
    fn into_response(self) -> http::Response<Self::Body>;
}

impl<T> IntoResponse for T
where
    T: serde::Serialize,
{
    type Buf = Cursor<Bytes>;
    type Body = Once<Self::Buf, ::Error>;

    fn into_response(self) -> http::Response<Self::Body> {
        // TODO: Improve and handle errors
        let body = ::serde_json::to_vec(&self).unwrap();
        let body = Cursor::new(Bytes::from(body));

        http::Response::builder()
            // Customize response
            .status(200)
            // This is not the right content type
            .header("content-type", "text/plain")
            .body(stream::once(Ok(body)))
            .unwrap()
    }
}
