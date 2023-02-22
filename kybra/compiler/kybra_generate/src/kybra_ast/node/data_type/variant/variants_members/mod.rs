use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{py_ast::kybra_types::KybraExpr, source_map::SourceMapped};
use cdk_framework::{act::node::data_type::variant::Member, ToDataType};

mod errors;
mod warnings;

impl SourceMapped<'_, Located<StmtKind>> {
    pub fn as_variant_member(&self) -> Member {
        match &self.node.node {
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                ..
            } => {
                match value {
                    Some(_) => eprintln!("{}", self.variant_default_value_warning()),
                    None => (),
                }
                let name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.variant_target_must_be_a_name_error()),
                };
                let type_ = KybraExpr {
                    located_expr: &annotation,
                    source_map: self.source_map.clone(),
                }
                .to_data_type();
                Member { name, type_ }
            }
            _ => panic!("{}", self.invalid_variant_member_error()),
        }
    }
}
