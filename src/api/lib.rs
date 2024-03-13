use crate::api::structs::ApiHandler;

pub fn setup() {
    println!("Init api handler!!");
    let api = ApiHandler {
        name: "Hello World",
        number: 10,
    };
    api.new();

    println!("name {} number {}", api.name, api.number);
}
