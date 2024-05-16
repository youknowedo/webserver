use std::vec;

use crate::{
    status::redirect::redirect_with_headers,
    template::{button_submit, form, h1, input, response_with_title, text, Response},
    RequestData,
};

pub fn page(data: RequestData) -> Response {
    if data.queries.contains_key("username") && data.queries.contains_key("password") {
        let username = data.queries.get("username").unwrap();
        let password = data.queries.get("password").unwrap();

        if *username == "admin" && *password == "password" {
            return redirect_with_headers(
                "/private",
                vec![("Set-Cookie".to_string(), format!("hasSession={}", username))]
                    .into_iter()
                    .collect(),
            );
        }
    }

    response_with_title(
        "200 OK",
        "Login",
        form(
            "/login",
            vec![
                h1("Login"),
                text("Username"),
                input("text", "username"),
                text("Password"),
                input("password", "password"),
                button_submit("Submit"),
            ],
        )
        .render(),
    )
}
