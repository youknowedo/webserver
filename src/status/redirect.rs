use std::collections::HashMap;

use crate::template::{response_without_body, Response};

pub fn redirect(location: &str) -> Response {
    response_without_body(
        "307 Temporary Redirect",
        vec![("Location", location)]
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    )
}
pub fn redirect_with_headers(location: &str, headers: HashMap<String, String>) -> Response {
    response_without_body(
        "307 Temporary Redirect",
        vec![("Location", location)]
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .chain(headers.iter().map(|(k, v)| (k.clone(), v.clone())))
            .collect(),
    )
}
