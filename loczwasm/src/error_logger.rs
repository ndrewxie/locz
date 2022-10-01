use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/error_logger.js")]
extern "C" {
    pub fn add_message(msgtype: &str, msgorigin: &str, message: &str);
    pub fn pause_panic();
}

#[macro_export]
macro_rules! wasmlog {
    ($($arg:tt)*) => {
        $crate::wasm_interop::add_message(
            "LOG(WASM)",
            &format!("{}:{}:{}", file!(), line!(), column!()),
            &format!($($arg)*)
        );
    }
}

pub fn init_panic() {
    panic::set_hook(Box::new(|panic_info| {
        add_message(
            "PANIC(WASM)",
            &panic_info
                .location()
                .map(|x| format!("{}:{}:{}", x.file(), x.line(), x.column()))
                .unwrap_or("Unk".to_owned()),
            panic_info
                .payload()
                .downcast_ref::<&str>()
                .unwrap_or(&"Unk"),
        );
        pause_panic();
    }));
}
