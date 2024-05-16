use std::vec;

use crate::{status::redirect::redirect_with_headers, template::Response, RequestData};

pub fn page(_: RequestData) -> Response {
    redirect_with_headers(
        "/",
        vec![(
            "Set-Cookie".to_string(),
            "hasSession=; Max-Age=0".to_string(),
        )]
        .into_iter()
        .collect(),
    )
}
