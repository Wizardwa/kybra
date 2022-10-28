use rustpython_parser::ast::ExprKind;

use crate::cdk_act::{
    nodes::data_type_nodes::{ActPrimitiveLit, ActTypeRef, ActTypeRefLit, LiteralOrTypeAlias},
    ActDataType, ToActDataType,
};

use super::KybraExpr;

impl ToActDataType for KybraExpr<'_> {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> ActDataType {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => ActPrimitiveLit::Blob.to_act_data_type(alias_name),
                "empty" => ActPrimitiveLit::Empty.to_act_data_type(alias_name),
                "float64" => ActPrimitiveLit::Float64.to_act_data_type(alias_name),
                "float32" => ActPrimitiveLit::Float32.to_act_data_type(alias_name),
                "int" => ActPrimitiveLit::Int.to_act_data_type(alias_name),
                "int64" => ActPrimitiveLit::Int64.to_act_data_type(alias_name),
                "int32" => ActPrimitiveLit::Int32.to_act_data_type(alias_name),
                "int16" => ActPrimitiveLit::Int16.to_act_data_type(alias_name),
                "int8" => ActPrimitiveLit::Int8.to_act_data_type(alias_name),
                "nat" => ActPrimitiveLit::Nat.to_act_data_type(alias_name),
                "nat64" => ActPrimitiveLit::Nat64.to_act_data_type(alias_name),
                "nat32" => ActPrimitiveLit::Nat32.to_act_data_type(alias_name),
                "nat16" => ActPrimitiveLit::Nat16.to_act_data_type(alias_name),
                "nat8" => ActPrimitiveLit::Nat8.to_act_data_type(alias_name),
                "null" => ActPrimitiveLit::Null.to_act_data_type(alias_name),
                "Principal" => ActPrimitiveLit::Principal.to_act_data_type(alias_name),
                "bool" => ActPrimitiveLit::Bool.to_act_data_type(alias_name),
                "reserved" => ActPrimitiveLit::Reserved.to_act_data_type(alias_name),
                "str" => ActPrimitiveLit::String.to_act_data_type(alias_name),
                "text" => ActPrimitiveLit::String.to_act_data_type(alias_name),
                _ => ActDataType::TypeRef(ActTypeRef {
                    act_type: LiteralOrTypeAlias::Literal(ActTypeRefLit {
                        name: id.to_string(),
                    }),
                }),
            },
            ExprKind::BoolOp { op, values } => todo!(),
            ExprKind::NamedExpr { target, value } => todo!(),
            ExprKind::BinOp { left, op, right } => todo!(),
            ExprKind::UnaryOp { op, operand } => todo!(),
            ExprKind::Lambda { args, body } => todo!(),
            ExprKind::IfExp { test, body, orelse } => todo!(),
            ExprKind::Dict { keys, values } => todo!(),
            ExprKind::Set { elts } => todo!(),
            ExprKind::ListComp { elt, generators } => todo!(),
            ExprKind::SetComp { elt, generators } => todo!(),
            ExprKind::DictComp {
                key,
                value,
                generators,
            } => todo!(),
            ExprKind::GeneratorExp { elt, generators } => todo!(),
            ExprKind::Await { value } => todo!(),
            ExprKind::Yield { value } => todo!(),
            ExprKind::YieldFrom { value } => todo!(),
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => todo!(),
            ExprKind::Call {
                func,
                args,
                keywords,
            } => todo!(),
            ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => todo!(),
            ExprKind::JoinedStr { values } => todo!(),
            ExprKind::Constant { value, kind } => todo!(),
            ExprKind::Attribute { value, attr, ctx } => todo!(),
            ExprKind::Subscript { value, slice, ctx } => todo!("What is a subscript"),
            ExprKind::Starred { value, ctx } => todo!(),
            ExprKind::List { elts, ctx } => todo!(),
            ExprKind::Tuple { elts, ctx } => todo!(),
            ExprKind::Slice { lower, upper, step } => todo!(),
        }
    }
}
