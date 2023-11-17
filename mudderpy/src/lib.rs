use mudder::SymbolTable as NativeSymbolTable;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass]
pub struct SymbolTable {
    table: NativeSymbolTable,
}

#[pymethods]
impl SymbolTable {
    #[new]
    fn new(symbols: &str) -> Self {
        Self {
            table: NativeSymbolTable::from_str(symbols),
        }
    }

    pub fn mudder(&self, amount: usize, a: Option<&str>, b: Option<&str>) -> PyResult<Vec<String>> {
        match self.table.mudder(amount, a, b) {
            Ok(r) => Ok(r),
            Err(e) => Err(PyRuntimeError::new_err(e)),
        }
    }

    pub fn mudder_one(&self, a: Option<&str>, b: Option<&str>) -> PyResult<String> {
        match self.table.mudder_one(a, b) {
            Ok(r) => Ok(r),
            Err(e) => Err(PyRuntimeError::new_err(e)),
        }
    }

    #[staticmethod]
    pub fn decimal() -> PyResult<Self> {
        Ok(Self {
            table: NativeSymbolTable::decimal(),
        })
    }

    #[staticmethod]
    pub fn alphabet() -> PyResult<Self> {
        Ok(Self {
            table: NativeSymbolTable::alphabetic(),
        })
    }

    #[staticmethod]
    pub fn base36() -> PyResult<Self> {
        Ok(Self {
            table: NativeSymbolTable::base36(),
        })
    }

    #[staticmethod]
    pub fn base62() -> PyResult<Self> {
        Ok(Self {
            table: NativeSymbolTable::base62(),
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn mudderpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SymbolTable>()?;
    Ok(())
}
