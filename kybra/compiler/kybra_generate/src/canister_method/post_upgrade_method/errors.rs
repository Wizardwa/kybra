use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_post_upgrade_method_allowed_error(&self) -> Error {
        Error::MultiplePostUpgrade(self.create_error_message(
            "Only one inspect message allowed",
            "Duplicate inspect here",
            None,
        ))
    }

    pub fn post_upgrade_method_must_return_void_error(&self) -> Error {
        Error::ReturnTypeMustBeVoid(self.create_error_message(
            "PostUpgrade method must have an explicit void return type annotation",
            "",
            None,
        ))
    }
}
