pub fn generate_generic_impls() -> proc_macro2::TokenStream {
    quote::quote! {
        impl<T> CdkActTryFromVmValue<Box<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Box<T>, CdkActTryFromVmValueError> {
                match self.try_from_vm_value(vm) {
                    Ok(value) => Ok(Box::new(value)),
                    Err(err) => Err(err)
                }
            }
        }

        impl<T> CdkActTryFromVmValue<Option<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Option<T>, CdkActTryFromVmValueError> {
                // TODO make sure this works, start using vm.ctx.none() instead of ().to_pyobject(vm)
                if self.is(&vm.ctx.none()) {
                    Ok(None)
                }
                else {
                    match self.try_from_vm_value(vm) {
                        Ok(value) => Ok(Some(value)),
                        Err(err) => Err(err)
                    }
                }
            }
        }
    }
}