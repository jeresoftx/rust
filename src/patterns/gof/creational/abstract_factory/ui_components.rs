/// Contrato publico `Button` que desacopla las piezas del ejemplo.
pub trait Button {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, label: &str) -> String;
}

/// Contrato publico `TextInput` que desacopla las piezas del ejemplo.
pub trait TextInput {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, placeholder: &str) -> String;
}

/// Contrato publico `UiFactory` que desacopla las piezas del ejemplo.
pub trait UiFactory {
    /// Operacion `create button` definida por la abstraccion del ejemplo.
    fn create_button(&self) -> Box<dyn Button>;
    /// Operacion `create text input` definida por la abstraccion del ejemplo.
    fn create_text_input(&self) -> Box<dyn TextInput>;
    /// Operacion `join form fields` definida por la abstraccion del ejemplo.
    fn join_form_fields(&self, button: String, input: String) -> String;
}

/// Tipo publico `ConsoleUiFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct ConsoleUiFactory;

impl UiFactory for ConsoleUiFactory {
    /// Operacion `create button` definida por la abstraccion del ejemplo.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(ConsoleButton)
    }

    /// Operacion `create text input` definida por la abstraccion del ejemplo.
    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(ConsoleTextInput)
    }

    /// Operacion `join form fields` definida por la abstraccion del ejemplo.
    fn join_form_fields(&self, button: String, input: String) -> String {
        format!("{button}\n{input}")
    }
}

/// Tipo publico `WebUiFactory` usado por el ejemplo para expresar el dominio del patron.
pub struct WebUiFactory;

impl UiFactory for WebUiFactory {
    /// Operacion `create button` definida por la abstraccion del ejemplo.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }

    /// Operacion `create text input` definida por la abstraccion del ejemplo.
    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(HtmlTextInput)
    }

    /// Operacion `join form fields` definida por la abstraccion del ejemplo.
    fn join_form_fields(&self, button: String, input: String) -> String {
        format!("{button}{input}")
    }
}

struct ConsoleButton;

impl Button for ConsoleButton {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, label: &str) -> String {
        format!("[ {label} ]")
    }
}

struct ConsoleTextInput;

impl TextInput for ConsoleTextInput {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, placeholder: &str) -> String {
        format!("> {placeholder}")
    }
}

struct HtmlButton;

impl Button for HtmlButton {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, label: &str) -> String {
        format!("<button>{label}</button>")
    }
}

struct HtmlTextInput;

impl TextInput for HtmlTextInput {
    /// Operacion `render` definida por la abstraccion del ejemplo.
    fn render(&self, placeholder: &str) -> String {
        format!(r#"<input placeholder="{placeholder}" />"#)
    }
}

/// Modela la operacion `render login form` dentro del ejemplo del patron.
pub fn render_login_form(factory: &dyn UiFactory) -> String {
    let button = factory.create_button();
    let input = factory.create_text_input();

    factory.join_form_fields(button.render("Login"), input.render("username"))
}
