# Chassis

*Compile-time dependency injector for Rust inspired by Dagger 2*

## Goals
* Detect errors at compile time like missing dependencies or cyclic dependencies
* No need to annotate your classes (support for third-party classes)
* No required usage of `std::sync::Arc`
* Zero overhead: Fast as hand-written code
    * No use of runtime type information

## Features

* Unscoped: create a new instance everytime
    * default
    * Type must not implement `Clone`
* Singletons: only one instance per component
    * Type must implement `Clone`
    * Created when component is created

## Example
```rust
use std::rc::Rc;

// define your business logic

/// printer trait
pub trait Printer {
    fn print(&self, input: &str);
}

/// a printer implementation
pub struct StdoutPrinter;

impl Printer for StdoutPrinter {
    fn print(&self, input: &str) {
        println!("{}", input);
    }
}

/// greeter for messages
pub struct Greeter {
    message: String,
    printer: Rc<dyn Printer>,
}

impl Greeter {
    /// constructor with dependencies
    pub fn new(message: String, printer: Rc<dyn Printer>) -> Self {
        Self { message, printer }
    }

    /// your business logic
    pub fn say_hello(&self) {
        self.printer.print(&self.message);
    }
}

/// module that is parsed to create the dependency injection
/// code
#[chassis::integration]
mod integration {
    use super::*;

    pub struct DemoModule;

    /// use strong types when in need to distinguish
    pub struct Message(String);

    /// Define how to create your dependencies
    impl DemoModule {
        #[singleton]
        pub fn provide_printer() -> Rc<dyn Printer> {
            Rc::new(StdoutPrinter)
        }

        pub fn provide_message() -> Message {
            Message("Hello World".to_string())
        }

        pub fn provide_greeter(
            message: Message,
            printer: Rc<dyn Printer>
        ) -> Greeter {
            Greeter::new(message.0, printer)
        }
    }

    /// Define which dependencies you need.
    ///
    /// A struct `DemoComponentImpl` will be created for
    /// you which implements `DemoComponent`.
    pub trait DemoComponent {
        /// request the to create injection code for 
        /// our main class `Greeter`
        fn resolve_greeter(&self) -> Greeter;
    }
}

fn main() {
    // import component trait
    use crate::integration::DemoComponent;

    // use generated component implementation
    let injector = integration::DemoComponentImpl::new();

    // Resolve main dependency
    // Note: it can not fail at runtime!
    let greeter = injector.resolve_greeter();

    // enjoy!
    greeter.say_hello();
}
```

### Missing features
* Request reference (access singletons without cloning)
* Lazy singletons (create singletons when needed)
* Lazy requests (request a factory instead of concrete type)
* Optional requests (only get it when it exists)
* Multiple provider (useful for plugins)
* Failable module functions (return `Result` in module)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.