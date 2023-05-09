use cdk_framework::{act::node::candid, traits::CollectResults};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CollectResults as OtherCollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

use super::errors::{InvalidTarget, NotExactlyOneTarget};

impl PyAst {
    pub fn build_type_aliases(&self) -> Result<Vec<candid::TypeAlias>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

struct TypeAlias<'a> {
    target: &'a Located<ExprKind>,
    value: &'a Located<ExprKind>,
}

impl SourceMapped<&Located<StmtKind>> {
    fn get_type_alias(&self) -> Result<Option<TypeAlias>, Error> {
        if let StmtKind::Assign { value, .. }
        | StmtKind::AnnAssign {
            value: Some(value), ..
        } = &self.node
        {
            if let ExprKind::Subscript { value, .. } = &value.node {
                if let ExprKind::Name { id, .. } = &value.node {
                    match id == "Alias" {
                        true => {
                            let (assign_target, assign_value) = match &self.node {
                                StmtKind::Assign { targets, value, .. } => {
                                    if targets.len() != 1 {
                                        return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                                    }
                                    (&targets[0], value)
                                }
                                StmtKind::AnnAssign {
                                    target,
                                    value: Some(value),
                                    ..
                                } => (target.as_ref(), value),
                                _ => return Ok(None),
                            };
                            return Ok(Some(TypeAlias {
                                target: assign_target,
                                value: assign_value,
                            }));
                        }
                        false => return Ok(None),
                    }
                }
            }
        }
        Ok(None)
    }

    fn as_type_alias(&self) -> Result<Option<candid::TypeAlias>, Vec<Error>> {
        let (assign_target, assign_value) =
            match self.get_type_alias().map_err(Into::<Vec<Error>>::into)? {
                Some(type_alias) => (type_alias.target, type_alias.value),
                None => return Ok(None),
            };

        let (alias_name, enclosed_expr) = (
            match &assign_target.node {
                ExprKind::Name { id, .. } => Ok(id.clone()),
                _ => Err(InvalidTarget::err_from_stmt(self).into()),
            },
            match &assign_value.node {
                ExprKind::Subscript { slice, .. } => Ok(slice),
                _ => Err(Unreachable::error().into()),
            },
        )
            .collect_results()?;

        let enclosed_type =
            SourceMapped::new(enclosed_expr.as_ref(), self.source_map.clone()).to_candid_type()?;
        Ok(Some(candid::TypeAlias {
            name: alias_name,
            aliased_type: Box::new(enclosed_type),
            type_params: vec![].into(),
        }))
    }
}
