use undoc::render::{JsonFormat, RenderOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OfficeDocument {
    #[allow(dead_code)]
    pub(crate) inner: undoc::Document,
}

#[wasm_bindgen]
impl OfficeDocument {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(data: &[u8]) -> Result<OfficeDocument, JsValue> {
        undoc::parse_bytes(data)
            .map(|inner| OfficeDocument { inner })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toMarkdown)]
    pub fn to_markdown(&self) -> Result<String, JsValue> {
        let opts = RenderOptions::default();
        undoc::render::to_markdown(&self.inner, &opts)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toText)]
    pub fn to_text(&self) -> Result<String, JsValue> {
        let opts = RenderOptions::default();
        undoc::render::to_text(&self.inner, &opts)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        undoc::render::to_json(&self.inner, JsonFormat::Compact)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn format(&self) -> String {
        self.inner.format.extension().to_string()
    }

    pub fn metadata(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.inner.metadata)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_node_experimental);

    #[wasm_bindgen_test]
    fn test_from_bytes_invalid_returns_err() {
        let result = OfficeDocument::from_bytes(b"not an office file");
        assert!(result.is_err());
    }
}
