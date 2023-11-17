use mudder::SymbolTable as NativeSymbolTable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SymbolTable {
    table: NativeSymbolTable,
}

#[wasm_bindgen]
impl SymbolTable {
    #[wasm_bindgen(constructor)]
    pub fn new(symbols: &str) -> Self {
        Self {
            table: NativeSymbolTable::from_str(symbols),
        }
    }

    pub fn mudder(
        &self,
        amount: usize,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<Vec<String>, JsValue> {
        match self.table.mudder(
            amount,
            start.as_ref().map(|s| &*s.as_str()),
            end.as_ref().map(|s| &*s.as_str()),
        ) {
            Ok(r) => Ok(r),
            Err(e) => Err(JsValue::from_str(e)),
        }
    }

    pub fn mudder_one(
        &self,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<String, JsValue> {
        match self.table.mudder_one(
            start.as_ref().map(|s| &*s.as_str()),
            end.as_ref().map(|s| &*s.as_str()),
        ) {
            Ok(r) => Ok(r),
            Err(e) => Err(JsValue::from_str(e)),
        }
    }

    pub fn decimal() -> Result<SymbolTable, JsValue> {
        Ok(Self {
            table: NativeSymbolTable::decimal(),
        })
    }

    pub fn alphabetic() -> Result<SymbolTable, JsValue> {
        Ok(Self {
            table: NativeSymbolTable::alphabetic(),
        })
    }

    pub fn base36() -> Result<SymbolTable, JsValue> {
        Ok(Self {
            table: NativeSymbolTable::base36(),
        })
    }

    pub fn base62() -> Result<SymbolTable, JsValue> {
        Ok(Self {
            table: NativeSymbolTable::base62(),
        })
    }

    pub fn hex() -> Result<SymbolTable, JsValue> {
        Ok(Self {
            table: NativeSymbolTable::hex(),
        })
    }
}
