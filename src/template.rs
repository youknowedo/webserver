use std::collections::HashMap;

pub trait Element {
    fn render(&self) -> String;
}

#[derive(Clone)]
pub struct Response {
    pub status: String,
    pub title: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: String,
}
pub fn response(status: &str, body: String) -> Response {
    Response {
        status: status.to_string(),
        title: None,
        headers: None,
        body: body.to_string(),
    }
}
pub fn response_without_body(status: &str, headers: HashMap<String, String>) -> Response {
    Response {
        status: status.to_string(),
        title: None,
        headers: Some(
            headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ),
        body: "".to_string(),
    }
}
pub fn response_without_body_and_header(status: &str) -> Response {
    Response {
        status: status.to_string(),
        title: None,
        headers: None,
        body: "".to_string(),
    }
}
pub fn response_with_title(status: &str, title: &str, body: String) -> Response {
    Response {
        status: status.to_string(),
        title: Some(title.to_string()),
        headers: None,
        body: body.to_string(),
    }
}

pub struct DivElement {
    children: Vec<Box<dyn Element>>,
}
impl Element for DivElement {
    fn render(&self) -> String {
        let children: Vec<String> = self.children.iter().map(|child| child.render()).collect();
        format!("<div>{}</div>", children.join(""))
    }
}
pub fn div(children: Vec<Box<dyn Element>>) -> Box<dyn Element> {
    Box::new(DivElement { children })
}

enum TextElementTypes {
    TEXT,
    P,
    H1,
}
pub struct TextElement {
    content: String,
    element: TextElementTypes,
}
impl Element for TextElement {
    fn render(&self) -> String {
        match self.element {
            TextElementTypes::TEXT => self.content.clone(),
            TextElementTypes::P => format!("<p>{}</p>", self.content),
            TextElementTypes::H1 => format!("<h1>{}</h1>", self.content),
        }
    }
}
pub fn text(content: &str) -> Box<dyn Element> {
    Box::new(TextElement {
        content: content.to_string(),
        element: TextElementTypes::TEXT,
    })
}
pub fn p(content: &str) -> Box<dyn Element> {
    Box::new(TextElement {
        content: content.to_string(),
        element: TextElementTypes::P,
    })
}
pub fn h1(content: &str) -> Box<dyn Element> {
    Box::new(TextElement {
        content: content.to_string(),
        element: TextElementTypes::H1,
    })
}

pub struct FormElement {
    action: String,
    children: Vec<Box<dyn Element>>,
}
impl Element for FormElement {
    fn render(&self) -> String {
        let children: Vec<String> = self.children.iter().map(|child| child.render()).collect();
        format!(
            "<form action='{}'>{}</form>",
            self.action,
            children.join("")
        )
    }
}
pub fn form(action: &str, children: Vec<Box<dyn Element>>) -> Box<dyn Element> {
    Box::new(FormElement {
        action: action.to_string(),
        children,
    })
}

pub struct InputElement {
    input_type: String,
    input_name: String,
}
impl Element for InputElement {
    fn render(&self) -> String {
        format!(
            "<input type=\"{}\" name=\"{}\" />",
            self.input_type, self.input_name
        )
    }
}
pub fn input(input_type: &str, input_name: &str) -> Box<dyn Element> {
    Box::new(InputElement {
        input_type: input_type.to_string(),
        input_name: input_name.to_string(),
    })
}

pub struct ButtonElement {
    button_text: String,
    on_click: String,
}
impl Element for ButtonElement {
    fn render(&self) -> String {
        format!(
            "<button onclick=\"{}\">{}</button>",
            self.on_click, self.button_text
        )
    }
}
pub fn button(button_text: &str, href: &str) -> Box<dyn Element> {
    Box::new(ButtonElement {
        button_text: button_text.to_string(),
        on_click: format!("window.location = '{}';", href),
    })
}
pub fn button_submit(button_text: &str) -> Box<dyn Element> {
    Box::new(ButtonElement {
        button_text: button_text.to_string(),
        on_click: "".to_string(),
    })
}
