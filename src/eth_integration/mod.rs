use wasm_bindgen::prelude::*;
use web_sys::window;

// Ethereum ile basit bir etkileşim örneği
#[wasm_bindgen]
pub fn connect_to_ethereum() -> Result<String, JsValue> {
    if let Some(win) = window() {
        // Burada Ethereum bağlantısını sağlama ve sonuç döndürme işlemleri yapılacak
        Ok("Connected to Ethereum!".to_string())
    } else {
        Err(JsValue::from_str("Could not access the window object."))
    }
}
