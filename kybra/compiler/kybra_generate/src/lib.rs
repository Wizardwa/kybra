use errors::KybraResult;
use proc_macro2::TokenStream;
use py_ast::PyAst;

pub mod async_result_handler;
pub mod body;
pub mod canister_method;
pub mod data_type;
pub mod debug;
mod errors;
pub mod external_canister;
pub mod get_name;
pub mod guard_function;
pub mod header;
pub mod ic_object;
pub mod keywords;
pub mod param;
pub mod py_ast;
pub mod source_map;
pub mod stable_b_tree_map_nodes;
pub mod tuple;
pub mod unwrap_rust_python_result;
pub mod vm_value_conversion;

pub use stable_b_tree_map_nodes::StableBTreeMapNode;

const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

pub fn get_python_keywords() -> Vec<String> {
    PYTHON_KEYWORDS
        .iter()
        .map(|keyword| keyword.to_string())
        .collect()
}

pub fn generate_canister(
    py_file_names: &Vec<&str>,
    entry_module_name: &str,
) -> KybraResult<TokenStream> {
    Ok(PyAst::new(py_file_names, entry_module_name)
        .to_act()?
        .to_token_stream())
}
