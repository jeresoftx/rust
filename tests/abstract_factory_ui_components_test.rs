use design_patterns_rust::patterns::gof::creational::abstract_factory::ui_components::{
    ConsoleUiFactory, WebUiFactory, render_login_form,
};

#[test]
fn abstract_factory_builds_console_ui_components() {
    let factory = ConsoleUiFactory;

    let form = render_login_form(&factory);

    assert_eq!(form, "[ Login ]\n> username");
}

#[test]
fn abstract_factory_builds_web_ui_components() {
    let factory = WebUiFactory;

    let form = render_login_form(&factory);

    assert_eq!(
        form,
        r#"<button>Login</button><input placeholder="username" />"#
    );
}
