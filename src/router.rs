use std::collections::HashMap;

use crate::{
    routes::{home, login, logout, private},
    status::not_found,
    template::Response,
    RequestData,
};

pub fn routes() -> HashMap<String, fn(data: RequestData) -> Response> {
    let mut routes: HashMap<String, fn(data: RequestData) -> Response> = HashMap::new();

    routes.insert("/".to_string(), home::page);
    routes.insert("/login".to_string(), login::page);
    routes.insert("/logout".to_string(), logout::page);
    routes.insert("/private".to_string(), private::page);

    routes.insert("404".to_string(), not_found);

    routes
}
