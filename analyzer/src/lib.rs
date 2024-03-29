mod analyze;
pub mod chart;
pub mod data;

#[allow(unused_imports, unused_macros)]
pub(crate) mod utils {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }

    macro_rules! console_log {
        ($($t:tt)*) => ($crate::utils::log(&std::format_args!($($t)*).to_string()));
    }

    pub(crate) use console_log;
}
