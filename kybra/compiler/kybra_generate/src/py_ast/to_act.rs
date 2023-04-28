use cdk_framework::{
    act::{CandidTypes, CanisterMethods, VmValueConversion},
    AbstractCanisterTree,
};

use super::PyAst;
use crate::{
    body,
    errors::KybraResult,
    header, keywords,
    vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
};

impl PyAst {
    pub fn to_act(&self) -> KybraResult<AbstractCanisterTree> {
        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes()?;
        let services = self.build_services()?;

        let (init_method, init_params, call_to_init_py_function) = self.build_init_method()?;

        let (post_upgrade_method, call_to_post_upgrade_py_function) =
            self.build_post_upgrade_method()?;

        let canister_methods = CanisterMethods {
            heartbeat_method: self.build_heartbeat_method()?,
            init_method: Some(init_method),
            inspect_message_method: self.build_inspect_method()?,
            post_upgrade_method: Some(post_upgrade_method),
            pre_upgrade_method: self.build_pre_upgrade_method()?,
            query_methods: self.build_query_methods()?,
            update_methods: self.build_update_methods()?,
        };

        let candid_types = CandidTypes {
            funcs: self.build_funcs()?,
            records: self.build_records()?,
            tuples: self.build_tuples()?,
            type_aliases: self.build_type_aliases()?,
            variants: self.build_variants()?,
            services: services.clone(),
        };

        let vm_value_conversion = VmValueConversion {
            try_from_vm_value_impls: try_into_vm_value_impls::generate(),
            try_into_vm_value_impls: try_from_vm_value_impls::generate(),
        };

        Ok(AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            header: header::generate(),
            body: body::generate(
                &canister_methods.update_methods,
                &canister_methods.query_methods,
                &services,
                &stable_b_tree_map_nodes,
                &self.entry_module_name,
                &init_params,
                call_to_init_py_function,
                call_to_post_upgrade_py_function,
            ),
            candid_types,
            canister_methods,
            guard_functions: self.build_guard_functions()?,
            vm_value_conversion,
            keywords: keywords::get_python_keywords(),
        })
    }
}
