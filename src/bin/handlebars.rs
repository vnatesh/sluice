extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

fn main() {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);

    // render without register
    println!("{}", reg.render_template("Hello {{name}}", &json!({"name": "foo"})).unwrap());

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}").unwrap();
    reg.register_template_file("tp1_2", "foobar").unwrap();
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"})).unwrap());
}
