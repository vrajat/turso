use turso_parser::ast::{self, Cmd, Expr, OneSelect, Stmt};

use crate::{walk_expr_mut, LimboError, WalkControl};

pub(crate) fn expand(args: &[Box<Expr>]) -> crate::Result<ast::Select> {
    let [after_lsn] = args else {
        crate::bail_parse_error!("pg_dbz() function must have exactly 1 argument");
    };
    let (Some(Cmd::Stmt(Stmt::Select(mut select))), _) =
        crate::dialect::sqlite::parse(include_str!("pg_dbz.sql"))?
    else {
        return Err(LimboError::InternalError(
            "pg_dbz expansion is not a SELECT".to_string(),
        ));
    };
    let OneSelect::Select {
        where_clause: Some(where_clause),
        ..
    } = &mut select.body.select
    else {
        return Err(LimboError::InternalError(
            "pg_dbz expansion has no WHERE clause".to_string(),
        ));
    };
    let mut replacements = 0;
    walk_expr_mut(where_clause, &mut |expr| {
        if matches!(expr, Expr::Variable(_)) {
            *expr = *after_lsn.clone();
            replacements += 1;
            return Ok(WalkControl::SkipChildren);
        }
        Ok(WalkControl::Continue)
    })?;
    if replacements != 1 {
        return Err(LimboError::InternalError(
            "pg_dbz expansion must have exactly one cursor parameter".to_string(),
        ));
    }
    Ok(select)
}
