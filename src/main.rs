#[tokio::main]
async fn main() {
    // Link sys and bind log crate
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, Embedded World!");
}
