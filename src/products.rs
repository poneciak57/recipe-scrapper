use cpython::{GILGuard, ObjectProtocol, PyModule, Python, ToPyObject};

pub struct ProductAnalyzer {
    gil: GILGuard,
    module: PyModule
}

impl ProductAnalyzer {

    pub fn new() -> ProductAnalyzer {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = py.import("analyzer_module")
            .expect("ERROR: Failed to load python module");
        ProductAnalyzer { module, gil }
    }

    pub fn analyze_product(&self, prod: &str) -> Option<String> {
        let py = self.gil.python();
        let args = (prod,).into_py_object(py);
        let revert_words_func = self.module.get(py, "revertWords")
            .expect("ERROR: Failed to get revertWords function from python module");

        revert_words_func
            .call(py, args, None).unwrap()
            .extract(py).unwrap()
    }
}