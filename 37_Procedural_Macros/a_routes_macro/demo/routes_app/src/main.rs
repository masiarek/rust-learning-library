use routes_macro::routes;

routes! {
    GET "/users" => fn list_users() -> String {
        String::from("every user")
    }
    GET "/users/{id}" => fn get_user(id: &str) -> String {
        String::from("user ") + id
    }
    POST "/users" => fn create_user() -> String {
        String::from("a new user")
    }
    GET "/users/{id}/orders/{order}" => fn get_order(id: &str, order: &str) -> String {
        String::from("order ") + order + " of user " + id
    }
}

fn main() {
    let requests = [
        ("GET", "/users"),
        ("GET", "/users/42"),
        ("POST", "/users"),
        ("GET", "/users/42/orders/7"),
        ("DELETE", "/users/42"),
        ("GET", "/teams"),
    ];
    for (method, path) in requests {
        match route(method, path) {
            Some((handler, body)) => println!("{method:<6} {path:<19} -> {handler}: {body}"),
            None => println!("{method:<6} {path:<19} -> no route"),
        }
    }
}
