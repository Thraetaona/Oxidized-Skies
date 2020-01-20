use wasm_bindgen::prelude::*;

pub enum Log {
  Info,
  Warning,
  Error,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn console_warn(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn console_error(s: &str);
}


fn log(message: &str, mode: Log) {
    match mode {
        Log::Info => console_log(&format!("[{}] {}", "INFO", message)),
        Log::Warning => console_warn(&format!("[{}] {}", "WARNING", message)),
        Log::Error => console_error(&format!("[{}] {}", "ERROR", message)),
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    log("Warnings are typically used for deprecations or recoverable errors.", Log::Warning);
}