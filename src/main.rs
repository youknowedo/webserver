use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use webserver::{router, status::not_found, RequestData};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let mut buf_reader = buf_reader.lines();
    let request_line = match buf_reader.next() {
        Some(line) => line.unwrap(),
        None => "GET 404 HTTP/1.1".to_string(),
    };
    let headers = buf_reader
        .take_while(|line| line.as_ref().unwrap() != "")
        .collect::<Vec<Result<String, _>>>()
        .iter()
        .map(|line| -> (String, String) {
            let header = line.as_ref().unwrap().split(": ").collect::<Vec<&str>>();
            (header[0].to_string(), header[1].to_string())
        })
        .collect::<HashMap<String, String>>();

    let routes = router::routes();

    let target = request_line.split(' ').collect::<Vec<&str>>()[1]
        .split('?')
        .collect::<Vec<&str>>();
    let route = target[0];
    let queries = target
        .get(1)
        .unwrap_or(&"")
        .split('&')
        .map(|q: &str| -> (String, String) {
            let query = q.split('=').collect::<Vec<&str>>();

            let key = query[0].to_string();
            let value = query.get(1).unwrap_or(&"").to_string();
            (key, value)
        })
        .collect::<HashMap<String, String>>();

    let data = RequestData { queries, headers };
    let response = match routes.get(route) {
        Some(r) => r(data),
        None => not_found(data),
    };

    let body = read_to_string("src/app.html")
        .unwrap()
        .replace("<web-body>", &response.body)
        .replace(
            "<web-title>",
            format!(
                "<title>{}</title>",
                match response.title {
                    Some(title) => format!("{} - Web server", title),
                    None => "Web server".to_string(),
                }
            )
            .as_str(),
        );

    let response_string = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}{}\r\n\r\n{}",
        response.status,
        body.len(),
        match response.headers {
            Some(headers) => headers
                .iter()
                .map(|(key, value)| format!("\r\n{}: {}", key, value))
                .collect::<String>(),
            None => "".to_string(),
        },
        body
    );

    stream.write_all(response_string.as_bytes()).unwrap();
}
