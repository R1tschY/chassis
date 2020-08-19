use std::error::Error;
use std::sync::Arc;

use chassis::{integration, BindAnnotation, Injector};

#[derive(Debug)]
struct Message;

impl BindAnnotation for Message {}

#[derive(Debug)]
struct Count;

impl BindAnnotation for Count {}

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

    pub struct DemoFactory;

    pub type Message = String;
    pub type Count = i32;

    #[static_module]
    impl DemoFactory {
        /* [lazy_singleton] */
        pub fn provide_count() -> Count {
            5
        }

        /* [eager_singleton] */
        pub fn provide_message() -> Message {
            "Hello World".to_string()
        }

        pub fn provide_greeter(message: Message, count: Count) -> Greeter {
            Greeter::new(message.to_string(), count)
        }
    }

    #[static_component(modules = [DemoFactory], send = false, sync = false)]
    pub trait DemoComponent {
        fn resolve_greeter(&self) -> Greeter;
    }
}

// TODO: Idea for introspection of modules: compile time visitor
// trait Visitable {
//     fn accept<T: Visitor>();
// }
//
// impl Visitable for int_mod::DemoModule {
//     fn accept<T: Visitor>() {
//         Visitor::visit::<Greeter>();
//     }
// }
//
// trait Visitor {
//     fn visit<T>();
// }

// GENERATED

/*struct ChassisDemoComponent {
    // module1: DemoModule,
}

impl ChassisDemoComponent {
    pub fn new() -> Self {
        Self {
            //module1: DemoModule::default()
        }
    }

    pub fn build(module1: DemoModule) -> Self {
        Self {
            //module1
        }
    }
}

impl DemoComponent for ChassisDemoComponent {
    fn resolve_greeter() -> Arc<Greeter> {
        Arc::new(DemoModule::provide_greeter(
            Arc::new(DemoModule::provide_message()),
            Arc::new(DemoModule::provide_count()),
        ))
    }
}*/

// /GENERATED

fn main() -> Result<(), Box<dyn Error>> {
    use crate::int_mod::DemoComponent;

    let injector = int_mod::DemoComponentImpl::new();
    let greeter = injector.resolve_greeter();
    greeter.say_hello();
    Ok(())
}
