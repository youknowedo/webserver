use std::collections::HashMap;

use crate::{
    status::unauthorized,
    template::{button, div, h1, p, response_with_title, Response},
    RequestData,
};

pub fn page(data: RequestData) -> Response {
    let cookie_string = data.headers.get("Cookie");

    if cookie_string.is_none() {
        return unauthorized();
    }

    let cookies: HashMap<&str, &str> = cookie_string
        .unwrap()
        .split("; ")
        .map(|c| -> (&str, &str) {
            let mut parts = c.split("=");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    if !cookies.contains_key("hasSession") {
        return unauthorized();
    }

    response_with_title(
        "200 OK",
        "Home",
        div(vec![
            h1("Private page!"),
            button("logout", "/logout"),
            p("This is a private page. Only logged in users can access this page."),
        ])
        .render(),
    )
}
