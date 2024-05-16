use std::{collections::HashMap, vec};

use crate::{
    template::{button, div, h1, p, response_with_title, Response},
    RequestData,
};

pub fn page(data: RequestData) -> Response {
    let cookie_string = data.headers.get("Cookie");

    let logged_in = if cookie_string.is_some() {
        let cookies: HashMap<&str, &str> = cookie_string
            .unwrap()
            .split("; ")
            .map(|c| -> (&str, &str) {
                let mut parts = c.split("=");
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();

        cookies.contains_key("hasSession")
    } else {
        false
    };

    response_with_title(
        "200 OK",
        "Home",
        div(vec![
            h1("Hello, world!"),
            div(vec![
                button("private", "/private"),
                if logged_in {
                    button("logout", "/logout")
                } else {
                    button("login", "/login")
                },
            ]),
            p("This is a text element."),
        ])
        .render(),
    )
}
