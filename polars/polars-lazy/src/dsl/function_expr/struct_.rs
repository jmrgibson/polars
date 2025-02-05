use polars_core::utils::slice_offsets;

use super::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StructFunction {
    FieldByIndex(i64),
    FieldByName(Arc<str>),
}

pub(super) fn get_by_index(s: &Series, index: i64) -> Result<Series> {
    let s = s.struct_()?;
    let (index, _) = slice_offsets(index, 0, s.fields().len());
    s.fields()
        .get(index)
        .cloned()
        .ok_or_else(|| PolarsError::ComputeError("index out of bounds in 'struct.field'".into()))
}
pub(super) fn get_by_name(s: &Series, name: Arc<str>) -> Result<Series> {
    let ca = s.struct_()?;
    ca.field_by_name(name.as_ref())
}
