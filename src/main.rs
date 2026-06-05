slint::include_modules!();

mod ui;
pub mod vehicle;
pub use vehicle::Vehicle;
fn main() {
    ui::run();
}
