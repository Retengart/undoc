mod document;

pub use document::OfficeDocument;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(data: &[u8]) -> Result<OfficeDocument, JsValue> {
    undoc::parse_bytes(data)
        .map(|inner| OfficeDocument { inner })
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_node_experimental);

    #[wasm_bindgen_test]
    fn test_parse_invalid_returns_error() {
        let result = parse(b"garbage data");
        assert!(result.is_err());
    }
}
