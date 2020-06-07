#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{get, routes, Rocket};

#[get("/hello/<name>")]
fn hello(name: String) -> String{
    format!("hello, {}", name)
}

#[get("/hello?<name>")]
fn query(name: Option<String>) -> String{
    name.map(|n| format!("hello, {}", n)).unwrap_or("hello".to_string())
}

fn rocket() -> Rocket {
    // rocket 支持重载还输的路由吗？
    rocket::ignite().mount("/", routes![hello, query])
}

fn main() {
    rocket().launch();
}
