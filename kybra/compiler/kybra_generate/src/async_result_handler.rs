use cdk_framework::act::{
    node::{candid::Service, Context},
    ToTypeAnnotation,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{keywords, tuple};

pub fn generate(services: &Vec<Service>) -> TokenStream {
    let call_match_arms = generate_call_match_arms(services);
    let call_with_payment_match_arms = generate_call_with_payment_match_arms(services);
    let call_with_payment128_match_arms = generate_call_with_payment128_match_arms(services);

    quote! {
        #[async_recursion::async_recursion(?Send)]
        async fn async_result_handler(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython::vm::PyObjectRef,
            arg: rustpython_vm::PyObjectRef
        ) -> rustpython::vm::PyObjectRef {
            if is_generator(vm, &py_object_ref) == false {
                return py_object_ref.clone();
            }

            let send_result = vm.call_method(&py_object_ref, "send", (arg.clone(),));
            let py_iter_return = rustpython_vm::protocol::PyIterReturn::from_pyresult(send_result, vm).unwrap_or_trap(vm);

            match py_iter_return {
                rustpython_vm::protocol::PyIterReturn::Return(returned_py_object_ref) => {
                    if is_generator(vm, &returned_py_object_ref) == true {
                        let recursed_py_object_ref = async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await;

                        return async_result_handler(vm, py_object_ref, recursed_py_object_ref).await;
                    }

                    let name: String = returned_py_object_ref.get_attr("name", vm).unwrap_or_trap(vm).try_from_vm_value(vm).unwrap_or_trap();
                    let args: Vec<rustpython_vm::PyObjectRef> = returned_py_object_ref.get_attr("args", vm).unwrap_or_trap(vm).try_into_value(vm).unwrap_or_trap(vm);

                    match &name[..] {
                        "call" => async_result_handler_call(vm, py_object_ref, &args).await,
                        "call_with_payment" => async_result_handler_call_with_payment(vm, py_object_ref, &args).await,
                        "call_with_payment128" => async_result_handler_call_with_payment128(vm, py_object_ref, &args).await,
                        "call_raw" => async_result_handler_call_raw(vm, py_object_ref, &args).await,
                        "call_raw128" => async_result_handler_call_raw128(vm, py_object_ref, &args).await,
                        _ => panic!("async operation not supported")
                    }
                },
                rustpython_vm::protocol::PyIterReturn::StopIteration(returned_py_object_ref_option) => match returned_py_object_ref_option {
                    Some(returned_py_object_ref) => returned_py_object_ref,
                    None => vm.ctx.none()
                }
            }
        }

        // TODO do this more officially, check if py_object_ref instanceof generator type
        fn is_generator(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef
        ) -> bool {
            if let Ok(_) = py_object_ref.get_attr("send", vm) {
                true
            }
            else {
                false
            }
        }

        async fn async_result_handler_call(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef,
            args: &Vec<rustpython_vm::PyObjectRef>
        ) -> rustpython_vm::PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap_or_trap();

            let cross_canister_call_function_name = format!("call_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_function_name[..] {
                #(#call_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        async fn async_result_handler_call_with_payment(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef,
            args: &Vec<rustpython_vm::PyObjectRef>
        ) -> rustpython_vm::PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap_or_trap();

            let cross_canister_call_with_payment_function_name = format!("call_with_payment_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_with_payment_function_name[..] {
                #(#call_with_payment_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        async fn async_result_handler_call_with_payment128(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef,
            args: &Vec<rustpython_vm::PyObjectRef>
        ) -> rustpython_vm::PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap_or_trap();

            let cross_canister_call_with_payment128_function_name = format!("call_with_payment128_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_with_payment128_function_name[..] {
                #(#call_with_payment128_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        async fn async_result_handler_call_raw(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef,
            args: &Vec<rustpython_vm::PyObjectRef>
        ) -> rustpython_vm::PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();
            let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap_or_trap();
            let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap_or_trap();
            let payment: u64 = args[3].clone().try_from_vm_value(vm).unwrap_or_trap();

            let call_raw_result = ic_cdk::api::call::call_raw(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment
            ).await;

            async_result_handler(vm, py_object_ref, create_call_result_instance(vm, call_raw_result)).await
        }

        async fn async_result_handler_call_raw128(
            vm: &rustpython::vm::VirtualMachine,
            py_object_ref: &rustpython_vm::PyObjectRef,
            args: &Vec<rustpython_vm::PyObjectRef>
        ) -> rustpython_vm::PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();
            let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap_or_trap();
            let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap_or_trap();
            let payment: u128 = args[3].clone().try_from_vm_value(vm).unwrap_or_trap();

            let call_raw_result = ic_cdk::api::call::call_raw128(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment
            ).await;

            async_result_handler(vm, py_object_ref, create_call_result_instance(vm, call_raw_result)).await
        }

        fn create_call_result_instance<T>(
            vm: &rustpython::vm::VirtualMachine,
            call_result: ic_cdk::api::call::CallResult<T>
        ) -> rustpython_vm::PyObjectRef
            where T: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
        {
            let call_result_class = vm.run_block_expr(
                vm.new_scope_with_builtins(),
                r#"
from kybra import CallResult

CallResult
                "#
            ).unwrap_or_trap(vm);

            match call_result {
                Ok(ok) => {
                    let method_result = vm.invoke(&call_result_class, (ok.try_into_vm_value(vm).unwrap_or_trap(), vm.ctx.none()));

                    method_result.unwrap_or_trap(vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("Ok", ok.try_into_vm_value(vm).unwrap_or_trap(), vm);

                    // dict
                },
                Err(err) => {
                    let err_string = format!("Rejection code {rejection_code}, {error_message}", rejection_code = (err.0 as i32).to_string(), error_message = err.1);

                    let method_result = vm.invoke(&call_result_class, (vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap_or_trap()));

                    method_result.unwrap_or_trap(vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("Err", err_string.try_into_vm_value(vm).unwrap_or_trap(), vm);

                    // dict
                }
            }
        }
    }
}

fn generate_call_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_name = format!(
                    "call_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_name_ident = format_ident!("{}", cross_canister_function_call_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap_or_trap();
                    }
                }).collect();

                let param_names = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                quote! {
                    #cross_canister_function_call_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();

                        #(#param_variable_definitions)*

                        create_call_result_instance(vm, #cross_canister_function_call_name_ident(canister_id_principal, #params).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}

fn generate_call_with_payment_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_with_payment_name = format!(
                    "call_with_payment_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_with_payment_name_ident = format_ident!("{}", cross_canister_function_call_with_payment_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap_or_trap();
                    }
                }).collect();

                let param_names: Vec<TokenStream> = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                let payment_index = method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u64 = args[#payment_index].clone().try_from_vm_value(vm).unwrap_or_trap(););

                quote! {
                    #cross_canister_function_call_with_payment_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        create_call_result_instance(vm, #cross_canister_function_call_with_payment_name_ident(canister_id_principal, #params, payment).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}

fn generate_call_with_payment128_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_with_payment128_name = format!(
                    "call_with_payment128_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_with_payment128_name_ident = format_ident!("{}", cross_canister_function_call_with_payment128_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap_or_trap();
                    }
                }).collect();

                let param_names: Vec<TokenStream> = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                let payment_index = method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u128 = args[#payment_index].clone().try_from_vm_value(vm).unwrap_or_trap(););

                quote! {
                    #cross_canister_function_call_with_payment128_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap_or_trap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        create_call_result_instance(vm, #cross_canister_function_call_with_payment128_name_ident(canister_id_principal, #params, payment).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}
