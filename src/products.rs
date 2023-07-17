use cpython::{GILGuard, ObjectProtocol, PyModule, PyObject, Python, ToPyObject};
use log::info;
use crate::prelude::Recipe;

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

    pub fn analyze_recipes(&self, recipes: &mut Vec<Recipe>) {
        let py = self.gil.python();
        let revert_words_func = self.module.get(py, "revertWords")
            .expect("ERROR: Failed to get revertWords function from python module");

        info!("Analyzing products ...");
        recipes.iter_mut().for_each(|recipe| {
            let products: Vec<String> = recipe.products.iter().map(|prod| {
                ProductAnalyzer::analyze_product(&py, &revert_words_func, prod).unwrap_or(format!("NIE ROZPOZNANO SKłADNIKA: {prod}"))
            }).collect();
            recipe.products = products;
        });
    }

    fn analyze_product(py: &Python, revert_words_func: &PyObject, prod: &str) -> Option<String> {
        let args = (prod,).into_py_object(*py);
        revert_words_func
            .call(*py, args, None).unwrap()
            .extract(*py).unwrap()
    }
}