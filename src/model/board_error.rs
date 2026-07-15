#[derive(Debug)]
pub enum BoardError {
    DefaultColumnCannotBeDeleted,
    DuplicateColumnId,
    DuplicateTaskId,
    NoColumnsWouldRemain,
    ColumnNotFound,
    ColumnContainsTasks,
}