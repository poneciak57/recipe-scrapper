use cpython::{GILGuard, ObjectProtocol, PyModule, PyObject, Python, ToPyObject};
use log::info;
use crate::prelude::Recipe;

#[derive(Debug, Clone)]
pub enum ProductState {
    ANALYZED(String),
    WAITING(String),
    UNRECOGNIZED(String),
}
impl ProductState {
    pub fn to_string(self) -> String {
        match self {
            ProductState::ANALYZED(s) => format!("{s}"),
            ProductState::WAITING(s) => format!("Error: Produkt oczekuje na analize: {s}"),
            ProductState::UNRECOGNIZED(s) => format!("NIE ROZPOZNANO SKłADNIKA: {s}"),
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            ProductState::ANALYZED(s) => format!("{s}"),
            ProductState::WAITING(s) => format!("Error: Produkt oczekuje na analize: {s}"),
            ProductState::UNRECOGNIZED(s) => format!("NIE ROZPOZNANO SKłADNIKA: {s}"),
        }
    }
}

pub struct ProductAnalyzer {
    gil: GILGuard,
    module: PyModule
}

impl ProductAnalyzer {

    pub fn new() -> ProductAnalyzer {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let module = py.import("analyzer_module")
            .expect("Error: Failed to load python module");
        ProductAnalyzer { module, gil }
    }

    pub fn analyze_recipes(&self, recipes: &mut Vec<Recipe>) {
        let py = self.gil.python();
        let revert_words_func = self.module.get(py, "revertWords")
            .expect("Error: Failed to get revertWords function from python module");

        info!("Analyzing products ...");
        recipes.iter_mut().for_each(|recipe| {
            let products: Vec<ProductState> = recipe.products.iter().map(|p| {
                if let ProductState::WAITING(prod) = p {
                    let analyzed = match ProductAnalyzer::analyze_product(&py, &revert_words_func, prod) {
                        None => ProductState::UNRECOGNIZED(prod.clone()),
                        Some(s) => ProductState::ANALYZED(s)
                    };
                    return analyzed
                }
                unreachable!()
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