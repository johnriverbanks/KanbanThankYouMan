use crate::model::column_error::ColumnError;

#[derive(Debug)]
pub enum BoardError {
    DefaultColumnCannotBeDeleted,
    DuplicateColumnId,
    DuplicateTaskId,
    NoColumnsWouldRemain,
    ColumnNotFound,
    ColumnContainsTasks,
    TaskNotFound,
    InvalidPosition,
}

impl From<ColumnError> for BoardError {
    fn from(error: ColumnError) -> Self {
        match error {
            ColumnError::TaskNotFound => BoardError::TaskNotFound,
            ColumnError::InvalidPosition => BoardError::InvalidPosition,
        }
    }
}