use crate::template::{response_with_title, Response};

pub fn unauthorized() -> Response {
    response_with_title(
        "401 Unauthorized",
        "401 Unauthorized",
        "401 Unauthorized".to_string(),
    )
}
