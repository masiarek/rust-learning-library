use routes_macro::routes;

routes! {
    GET "/users/{id}" => fn get_user() -> String {
        String::from("which user?")
    }
    GET "/users/{id}/orders" => fn list_orders(id: &str, page: &str) -> String {
        String::from("orders of user ") + id + ", page " + page
    }
    GET "/users/{user_id}" => fn find_user(user_id: &str) -> String {
        String::from("user ") + user_id
    }
}

fn main() {}
