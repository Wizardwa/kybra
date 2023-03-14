use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_set_certified_data(&self, data_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
            let data: Vec<u8> = data_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::set_certified_data(&data).try_into_vm_value(vm).unwrap();
        }
    }
}