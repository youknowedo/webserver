use crate::{
    template::{div, p, response_with_title, Response},
    RequestData,
};

pub fn not_found(_: RequestData) -> Response {
    response_with_title(
        "404 Not Found",
        "404 Not Found",
        div(vec![p("404 Not Found")]).render(),
    )
}
