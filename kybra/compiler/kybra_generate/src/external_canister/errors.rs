use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn class_with_not_function_defs_error(&self, canister_name: &String) -> Vec<Message> {
        let title = format!(
            "class \"{}\" should only contain function definitions. Please remove everything else.",
            canister_name
        );
        vec![self.create_error_message(title.as_str(), "", None)]
    }

    pub fn class_must_have_methods_error(&self, canister_name: &String) -> Vec<Message> {
        let title = format!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", canister_name);
        vec![self.create_error_message(title.as_str(), "", None)]
    }

    pub fn missing_decorator_error(
        &self,
        canister_name: &String,
        method_name: &String,
    ) -> Vec<Message> {
        let title = format!(
            "{}.{} is missing a @query or @update decorator. Please add it above the method",
            canister_name, method_name
        );
        vec![self.create_error_message(title.as_str(), "", None)]
    }

    pub fn too_many_decorators_error(
        &self,
        canister_name: &String,
        method_name: &String,
    ) -> Vec<Message> {
        let title = format!(
            "{}.{} has too many decorators. Please remove all but either @update or @query",
            canister_name, method_name
        );
        vec![self.create_error_message(title.as_str(), "", None)]
    }

    pub fn wrong_decorator_error(
        &self,
        canister_name: &String,
        method_name: &String,
        id: &String,
    ) -> Vec<Message> {
        let title = format!(
            "{}.{} has the wrong decorator: expected @update or @query, got \"@{}\"",
            canister_name, method_name, id
        );
        vec![self.create_error_message(title.as_str(), "", None)]
    }

    pub fn invalid_decorator_error(
        &self,
        canister_name: &String,
        method_name: &String,
    ) -> Vec<Message> {
        let title = format!(
            "{}.{} has an invalid decorator. Change it to either @update or @query",
            canister_name, method_name
        );
        vec![self.create_error_message(title.as_str(), "", None)]
    }
}
