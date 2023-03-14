use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_canister_balance(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::canister_balance().try_into_vm_value(vm).unwrap()
        }
    }
}