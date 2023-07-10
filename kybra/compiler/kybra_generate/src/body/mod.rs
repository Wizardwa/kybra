use cdk_framework::act::node::{
    candid::Service,
    canister_method::{QueryMethod, UpdateMethod},
};
use proc_macro2::TokenStream;

use crate::{ic_object, stable_b_tree_map_nodes::rust, StableBTreeMapNode};

mod async_result_handler;
mod call_global_python_function;
mod does_interpreter_exist;
mod unwrap_rust_python_result;
mod utils;

pub fn generate(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
    services: &Vec<Service>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> TokenStream {
    let ic_object = ic_object::generate(
        update_methods,
        query_methods,
        services,
        stable_b_tree_map_nodes,
    );
    let unwrap_rust_python_result = unwrap_rust_python_result::generate();
    let async_result_handler = async_result_handler::generate(&services);
    let stable_b_tree_map = rust::generate(stable_b_tree_map_nodes);
    let utils = utils::generate();
    let call_global_python_function = call_global_python_function::generate();
    let does_interpreter_exist = does_interpreter_exist::generate();

    quote::quote! {
        #ic_object
        #unwrap_rust_python_result
        #async_result_handler
        #stable_b_tree_map
        #utils
        #call_global_python_function
        #does_interpreter_exist

        #[ic_cdk_macros::query]
        fn export_candid() -> String {
            __export_service()
        }
    }
}
