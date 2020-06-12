use chassis::{module, BindAnnotation, Injector};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
struct Message;
impl BindAnnotation for Message {}

#[derive(Debug)]
struct Count;
impl BindAnnotation for Count {}

struct Greeter {
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

struct DemoModule;

#[module]
impl DemoModule {
    #[annotation(Count)]
    pub fn provide_count() -> i32 {
        5
    }

    #[annotation(Message)]
    pub fn provide_message() -> String {
        "Hello World".to_string()
    }

    #[rustfmt::skip]
    pub fn provide_greeter(
        #[annotation(Message)] message: Arc<String>,
        #[annotation(Count)] count: Arc<i32>
    ) -> Greeter {
        Greeter::new(message.to_string(), *count)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let injector = Injector::builder().module(DemoModule).build()?;
    let greeter = injector.resolve_type::<Greeter>().unwrap();
    greeter.say_hello();
    Ok(())
}
