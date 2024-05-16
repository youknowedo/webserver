use std::collections::HashMap;

pub mod router;
pub mod routes;
pub mod status;
pub mod template;

pub struct RequestData {
    pub queries: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}
