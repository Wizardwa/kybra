use cdk_framework::act::node::{
    candid::Primitive,
    canister_method::{CanisterMethodType, HeartbeatMethod},
    CandidType,
};

use super::rust;
use crate::{errors::KybraResult, py_ast::PyAst};

impl PyAst {
    pub fn build_heartbeat_method(&self) -> KybraResult<Option<HeartbeatMethod>> {
        let heartbeat_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Heartbeat);

        if heartbeat_function_defs.len() > 1 {
            return Err(heartbeat_function_defs
                .iter()
                .map(|heartbeat_function_def| {
                    heartbeat_function_def.only_one_heartbeat_allowed_error()
                })
                .collect());
        }

        let heartbeat_function_def_option = heartbeat_function_defs.get(0);

        Ok(
            if let Some(heartbeat_function_def) = heartbeat_function_def_option {
                if let CandidType::Primitive(primitive) =
                    heartbeat_function_def.build_return_type()?
                {
                    if let Primitive::Void = primitive {
                        ()
                    } else {
                        return Err(
                            heartbeat_function_def.heartbeat_method_must_return_void_error()
                        );
                    }
                }
                let body = rust::generate(heartbeat_function_def)?;
                Some(HeartbeatMethod {
                    body,
                    guard_function_name: None,
                })
            } else {
                None
            },
        )
    }
}
