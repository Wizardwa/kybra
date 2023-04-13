use proc_macro2::TokenStream;
use py_ast::PyAst;

pub mod async_result_handler;
pub mod body;
pub mod candid_type;
pub mod canister_method;
pub mod debug;
pub mod errors;
pub mod get_name;
pub mod guard_function;
pub mod header;
pub mod ic_object;
pub mod keywords;
pub mod kybra_modules_init;
pub mod method_utils;
pub mod py_ast;
pub mod source_map;
pub mod stable_b_tree_map_nodes;
pub mod tuple;
pub mod unwrap_rust_python_result;
pub mod vm_value_conversion;

pub use errors::Error;
pub use stable_b_tree_map_nodes::StableBTreeMapNode;

pub fn generate_canister(
    py_file_names: &Vec<&str>,
    entry_module_name: &str,
) -> Result<TokenStream, Vec<Error>> {
    PyAst::new(py_file_names, entry_module_name)
        .to_act()?
        .to_token_stream()
        .map_err(|cdkf_errors| cdkf_errors.into_iter().map(Error::from).collect())
}
