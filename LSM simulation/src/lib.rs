
#[cfg(target_arch = "wasm32")]
mod main;
#[cfg(target_arch = "wasm32")]
mod vec_tree;
#[cfg(target_arch = "wasm32")]
mod instance_data;
#[cfg(target_arch = "wasm32")]
mod model_functions;
#[cfg(target_arch = "wasm32")]
mod gui_run;

// Entry point for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");
    info!("eeeeee");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gui_run::run_gui_showcase().await;
    Ok(())
}