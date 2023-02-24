// mod errors;

// use rustpython_parser::ast::{ExprKind, StmtKind};

// use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};
// use cdk_framework::act::node::data_type::DataType;

// impl KybraStmt<'_> {
//     pub fn is_tuple(&self) -> bool {
//         match &self.stmt_kind.node {
//             StmtKind::Assign { value, .. } => KybraExpr {
//                 located_expr: value,
//                 source_map: self.source_map.clone(),
//             }
//             .is_tuple(),
//             _ => false,
//         }
//     }

//     pub fn as_tuple(&self) -> DataType {
//         match &self.stmt_kind.node {
//             StmtKind::Assign { targets, value, .. } => {
//                 if targets.len() > 1 {
//                     panic!("{}", self.multiple_targets_error())
//                 }
//                 let tuple_name = match &targets[0].node {
//                     ExprKind::Name { id, .. } => id,
//                     _ => panic!("{}", self.invalid_target_error()),
//                 };
//                 KybraExpr {
//                     located_expr: value,
//                     source_map: self.source_map.clone(),
//                 }
//                 .to_tuple(Some(tuple_name))
//             }
//             _ => panic!("{}", self.not_a_tuple_error()),
//         }
//     }
// }
