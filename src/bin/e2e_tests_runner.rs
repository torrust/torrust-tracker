//! Program to run E2E tests.
use torrust_tracker::console::ci::e2e;

fn main() {
    e2e::runner::run();
}
