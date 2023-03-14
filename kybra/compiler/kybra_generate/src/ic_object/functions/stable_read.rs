use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_stable_read(&self, offset_py_object_ref: PyObjectRef, length_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
            let length: u32 = length_py_object_ref.try_from_vm_value(vm).unwrap();

            let mut buf: Vec<u8> = vec![0; length as usize];
            ic_cdk::api::stable::stable_read(offset, &mut buf);
            buf.try_into_vm_value(vm).unwrap()
        }
    }
}