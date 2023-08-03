use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<f64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<f64, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to f64".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<_CdkFloat64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<_CdkFloat64, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(_CdkFloat64(value)),
                    Err(err) => Err(vm.new_type_error(
                        "Could not convert PyObjectRef to _CdkFloat64".to_string(),
                    )),
                }
            }
        }

        impl CdkActTryFromVmValue<f32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<f32, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to f32".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<_CdkFloat32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<_CdkFloat32, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(_CdkFloat32(value)),
                    Err(err) => Err(vm.new_type_error(
                        "Could not convert PyObjectRef to _CdkFloat32".to_string(),
                    )),
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Int, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<ic_cdk::export::candid::Int, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let int_result: Result<rustpython_vm::builtins::PyIntRef, _> =
                    self.try_into_value(vm);

                match int_result {
                    Ok(int) => Ok(ic_cdk::export::candid::Int(int.as_bigint().clone())),
                    Err(_) => Err(vm.new_type_error("PyObjectRef is not a PyIntRef".to_string())),
                }
            }
        }

        impl CdkActTryFromVmValue<i128, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i128, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to i128".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<i64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i64, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to i64".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<i32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i32, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to i32".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<i16, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i16, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to i16".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<i8, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i8, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to i8".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Nat, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<ic_cdk::export::candid::Nat, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let int_result: Result<rustpython_vm::builtins::PyIntRef, _> =
                    self.try_into_value(vm);

                match int_result {
                    Ok(int) => {
                        match ic_cdk::export::candid::Nat::from_str(&int.as_bigint().to_string()) {
                            // TODO probably not the best conversion
                            Ok(nat) => Ok(nat),
                            Err(_) => {
                                Err(vm.new_type_error("Could not convert value to nat".to_string()))
                            }
                        }
                    }
                    Err(_) => Err(vm.new_type_error("PyObjectRef is not a PyIntRef".to_string())),
                }
            }
        }

        impl CdkActTryFromVmValue<u128, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u128, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to u128".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<u64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u64, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to u64".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<usize, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<usize, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to usize".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<u32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u32, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to u32".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<u16, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u16, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to u16".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<u8, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u8, rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        Err(vm.new_type_error("Could not convert PyObjectRef to u8".to_string()))
                    }
                }
            }
        }
    }
}
