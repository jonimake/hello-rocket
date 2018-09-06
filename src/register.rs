use rocket_contrib::Template;

use std::collections::HashMap;

#[get("/register")]
fn show() -> Template {
    let ctx: HashMap<&str, &str> = HashMap::new();
    Template::render("register", ctx)
}