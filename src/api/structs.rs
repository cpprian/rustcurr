pub struct ApiHandler<'a> {
    pub name: &'a str,
    pub number: u32,
}

impl<'a> ApiHandler<'a> {
    pub fn new(&self) {
        println!("setup logic from ApiHandler");
    }

    pub fn default(&mut self) {
        self.name = "Hello";
        self.number = 10;
    }
}

#[cfg(test)]
mod tests {
    use super::ApiHandler;

    #[test]
    fn quick_test() {
        let mut api = ApiHandler {
            name: "World",
            number: 5,
        };

        api.default();
        assert_eq!("Hello", api.name);
        assert_eq!(10, api.number);
    }
}
