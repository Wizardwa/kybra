use cdk_act::{ToAct, ToTokenStream};
use generators::ic_object::generate_ic_object;
use py_ast::{KybraProgram, PyAst};
use quote::quote;
use rustpython_parser::parser::{self, Mode};
use source_map::SourceMap;

mod cdk_act;
mod errors;
mod generators;
mod py_ast;
mod source_map;

pub fn kybra_generate(
    main_py: &str,
    entry_module_name: &str,
) -> proc_macro2::token_stream::TokenStream {
    eprintln!("-------------------------------------------");
    eprintln!("--- STARTING ------------------------------");
    eprintln!("-------------------------------------------");
    let py_file_names = vec![main_py];
    let source_map = SourceMap {};
    let kybra_programs: Vec<KybraProgram> = py_file_names
        .iter()
        .map(|py_file_name| KybraProgram {
            program: parser::parse(py_file_name, Mode::Module, "").unwrap(),
            source_map: &source_map,
        })
        .collect();
    let canister_definition = PyAst {
        kybra_programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .to_kybra_ast()
    .to_act()
    .to_token_stream();
    eprintln!("-------------------------------------------");
    eprintln!("--- ENDING --------------------------------");
    eprintln!("-------------------------------------------");

    let ic_object = generate_ic_object();

    quote! {
        use rustpython_vm::{AsObject, builtins::{PyListRef, PyTupleRef, PyIntRef}, class::PyClassImpl, convert::ToPyObject, PyObjectRef, VirtualMachine};
        use rustpython_derive::{pyclass, PyPayload};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use std::str::FromStr;

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;

        #ic_object

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";

        #canister_definition

    }
}
