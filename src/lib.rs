use wasm_bindgen::prelude::*;
use web_sys::window;

// Bu işlev JavaScript tarafından çağrılacak ve bir uyarı gösterecektir.
#[wasm_bindgen]
pub fn greet() {
    if let Some(win) = window() {
        win.alert_with_message("Hello, WebAssembly!").unwrap();
    } else {
        web_sys::console::error_1(&"Pencere nesnesi alınamadı".into());
    }
}

// JavaScript tarafından çağrılacak başka bir işlev örneği
#[wasm_bindgen]
pub fn another_function() -> i32 {
    42
}

// Sıfıra bölme hatası durumunda bir hata döndüren ek bir işlev
#[wasm_bindgen]
pub fn divide(x: i32, y: i32) -> Result<i32, JsValue> {
    if y == 0 {
        Err(JsValue::from_str("Division by zero error"))
    } else {
        Ok(x / y)
    }
}
