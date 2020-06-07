use bytes::Bytes;

use crate::error::Error;
use crate::mssql::protocol::col_meta_data::ColumnData;
use crate::mssql::MssqlTypeInfo;

#[derive(Debug)]
pub(crate) struct Row {
    // TODO: Column names?
    // FIXME: Columns Vec should be an Arc<_>
    pub(crate) column_types: Vec<MssqlTypeInfo>,
    pub(crate) values: Vec<Option<Bytes>>,
}

impl Row {
    pub(crate) fn get(buf: &mut Bytes, columns: &[ColumnData]) -> Result<Self, Error> {
        let mut values = Vec::with_capacity(columns.len());
        let mut column_types = Vec::with_capacity(columns.len());

        for column in columns {
            column_types.push(MssqlTypeInfo(column.type_info.clone()));

            if column.type_info.is_null() {
                values.push(None);
            } else {
                values.push(Some(column.type_info.get_value(buf)));
            }
        }

        Ok(Self {
            values,
            column_types,
        })
    }
}