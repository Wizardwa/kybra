use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn stable64_read(
            &self,
            offset_py_object_ref: rustpython_vm::PyObjectRef,
            length_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap_or_trap();
            let length: u64 = length_py_object_ref.try_from_vm_value(vm).unwrap_or_trap();

            let mut buf: Vec<u8> = vec![0; length as usize];
            ic_cdk::api::stable::stable64_read(offset, &mut buf);
            buf.try_into_vm_value(vm).unwrap_or_trap()
        }
    }
}
