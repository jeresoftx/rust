pub trait Button {
    fn render(&self, label: &str) -> String;
}

pub trait TextInput {
    fn render(&self, placeholder: &str) -> String;
}

pub trait UiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_text_input(&self) -> Box<dyn TextInput>;
    fn join_form_fields(&self, button: String, input: String) -> String;
}

pub struct ConsoleUiFactory;

impl UiFactory for ConsoleUiFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(ConsoleButton)
    }

    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(ConsoleTextInput)
    }

    fn join_form_fields(&self, button: String, input: String) -> String {
        format!("{button}\n{input}")
    }
}

pub struct WebUiFactory;

impl UiFactory for WebUiFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }

    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(HtmlTextInput)
    }

    fn join_form_fields(&self, button: String, input: String) -> String {
        format!("{button}{input}")
    }
}

struct ConsoleButton;

impl Button for ConsoleButton {
    fn render(&self, label: &str) -> String {
        format!("[ {label} ]")
    }
}

struct ConsoleTextInput;

impl TextInput for ConsoleTextInput {
    fn render(&self, placeholder: &str) -> String {
        format!("> {placeholder}")
    }
}

struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self, label: &str) -> String {
        format!("<button>{label}</button>")
    }
}

struct HtmlTextInput;

impl TextInput for HtmlTextInput {
    fn render(&self, placeholder: &str) -> String {
        format!(r#"<input placeholder="{placeholder}" />"#)
    }
}

pub fn render_login_form(factory: &dyn UiFactory) -> String {
    let button = factory.create_button();
    let input = factory.create_text_input();

    factory.join_form_fields(button.render("Login"), input.render("username"))
}
