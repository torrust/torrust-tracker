use std::cell::RefCell;

use super::printer::{Printer, CLEAR_SCREEN};

pub struct Tracer {
    output: RefCell<String>,
}

impl Default for Tracer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tracer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            output: RefCell::new(String::new()),
        }
    }

    pub fn tracing(&self) -> String {
        self.output.borrow().clone()
    }
}

impl Printer for Tracer {
    fn clear(&self) {
        self.print(CLEAR_SCREEN);
    }

    fn print(&self, output: &str) {
        *self.output.borrow_mut() = format!("{}{}", self.output.borrow(), &output);
    }

    fn eprint(&self, output: &str) {
        *self.output.borrow_mut() = format!("{}{}", self.output.borrow(), &output);
    }

    fn println(&self, output: &str) {
        self.print(&format!("{}/n", &output));
    }

    fn eprintln(&self, output: &str) {
        self.eprint(&format!("{}/n", &output));
    }
}

#[cfg(test)]
mod tests {
    use crate::console::clients::checker::logger::Tracer;
    use crate::console::clients::checker::printer::{Printer, CLEAR_SCREEN};

    #[test]
    fn should_capture_the_clear_screen_command() {
        let console_logger = Tracer::new();

        console_logger.clear();

        assert_eq!(CLEAR_SCREEN, console_logger.tracing());
    }

    #[test]
    fn should_capture_the_print_command_output() {
        let console_logger = Tracer::new();

        console_logger.print("OUTPUT");

        assert_eq!("OUTPUT", console_logger.tracing());
    }
}
