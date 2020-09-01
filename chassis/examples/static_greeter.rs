use chassis::integration;

pub struct Greeter {
    message: String,
    count: i32,
}

impl Greeter {
    pub fn new(message: String, count: i32) -> Self {
        Self { message, count }
    }

    pub fn say_hello(&self) {
        for _ in 0..self.count {
            println!("{}", self.message);
        }
    }
}

#[integration]
mod int_mod {
    use super::*;

    pub struct DemoModule;

    pub struct Message(String);
    pub struct Count(i32);

    impl DemoModule {
        pub fn provide_count() -> Count {
            Count(5)
        }

        pub fn provide_message() -> Message {
            Message("Hello World".to_string())
        }

        pub fn provide_greeter(message: Message, count: Count) -> Greeter {
            Greeter::new(message.0, count.0)
        }
    }

    pub trait DemoComponent {
        fn resolve_greeter(&self) -> Greeter;
    }
}

fn main() {
    use crate::int_mod::DemoComponent;

    let injector = int_mod::DemoComponentImpl::new();
    let greeter = injector.resolve_greeter();
    greeter.say_hello();
}
