//! Polars lazy does not implement a pivot because it is impossible to know the schema without
//! materializing the whole dataset. This makes a pivot quite a terrible operation for performant
//! workflows. An optimization can never be pushed down passed a pivot.
//!
//! We can do a pivot on an eager `DataFrame` as that is already materialized. The code for the
//! pivot is here, because we want to be able to pass expressions to the pivot operation.
//!

use polars_core::frame::groupby::PivotAgg;
use polars_core::{frame::groupby::expr::PhysicalAggExpr, prelude::*};

use crate::physical_plan::exotic::{prepare_eval_expr, prepare_expression_for_context};
use crate::physical_plan::state::ExecutionState;
use crate::prelude::*;

impl PhysicalAggExpr for Expr {
    fn evaluate<'a>(&self, df: &DataFrame, groups: &'a GroupsProxy) -> Result<Series> {
        let state = ExecutionState::new();
        let dtype = df.get_columns()[0].dtype();
        let phys_expr = prepare_expression_for_context("", self, dtype, Context::Aggregation)?;
        phys_expr
            .evaluate_on_groups(df, groups, &state)
            .map(|mut ac| ac.aggregated())
    }

    fn root_name(&self) -> Result<&str> {
        Ok("")
    }
}

pub fn pivot<I0, S0, I1, S1, I2, S2>(
    df: &DataFrame,
    values: I0,
    index: I1,
    columns: I2,
    agg_expr: Expr,
    sort_columns: bool,
) -> Result<DataFrame>
where
    I0: IntoIterator<Item = S0>,
    S0: AsRef<str>,
    I1: IntoIterator<Item = S1>,
    S1: AsRef<str>,
    I2: IntoIterator<Item = S2>,
    S2: AsRef<str>,
{
    // make sure that the root column is replaced
    let expr = prepare_eval_expr(agg_expr);
    df.pivot(
        values,
        index,
        columns,
        PivotAgg::Expr(Arc::new(expr)),
        sort_columns,
    )
}

pub fn pivot_stable<I0, S0, I1, S1, I2, S2>(
    df: &DataFrame,
    values: I0,
    index: I1,
    columns: I2,
    agg_expr: Expr,
    sort_columns: bool,
) -> Result<DataFrame>
where
    I0: IntoIterator<Item = S0>,
    S0: AsRef<str>,
    I1: IntoIterator<Item = S1>,
    S1: AsRef<str>,
    I2: IntoIterator<Item = S2>,
    S2: AsRef<str>,
{
    // make sure that the root column is replaced
    let expr = prepare_eval_expr(agg_expr);
    df.pivot_stable(
        values,
        index,
        columns,
        PivotAgg::Expr(Arc::new(expr)),
        sort_columns,
    )
}
