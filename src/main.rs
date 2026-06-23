slint::include_modules!();

mod observer;
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

    observer::run();
}

#[cfg(not(feature = "web"))]
fn main() {
    observer::run();
}
