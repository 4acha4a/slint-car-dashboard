slint::include_modules!();

mod ui;
pub mod vehicle;
pub use vehicle::Vehicle;
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "web")]
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        web::run_web().await;
    });

    ui::run();
}

#[cfg(not(feature = "web"))]
fn main() {
    ui::run();
}
