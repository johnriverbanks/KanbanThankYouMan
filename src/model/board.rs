use std::collections::HashMap;
use crate::model::board_error::BoardError;
use crate::model::column::Column;
use crate::model::column_id::ColumnId;
use crate::model::tasks::Task;
use crate::model::task_draft::TaskDraft;
use crate::model::task_id::TaskId;

pub struct Board {
    columns: HashMap<ColumnId, Column>,
    column_order: Vec<ColumnId>,
    tasks: HashMap<TaskId, Task>,
    default_column: ColumnId,
}

impl Board {

    pub fn from_config(
        columns: HashMap<ColumnId, Column>,
        column_order: Vec<ColumnId>,
        default_column: ColumnId,
    ) -> Result<Self, BoardError> {
        if !columns.contains_key(&default_column) {
            return Err(BoardError::ColumnNotFound)
        }
        Ok(Self {
            columns,
            column_order,
            tasks: HashMap::new(),
            default_column,
        })
    }

    pub fn add_column(
        &mut self,
        column: Column,
    ) -> Result<(), BoardError> {
        //Error Here
        let id = column.id().clone();
        if self.columns.contains_key(&id) {
            return Err(BoardError::DuplicateColumnId)
        }
        self.column_order.push(id.clone());
        self.columns.insert(id, column);
        Ok(())
    }

    pub fn columns(&self) -> impl Iterator<Item = &Column> {
        self.column_order
            .iter()
            .filter_map(
                |id| self.columns.get(id)
            )
    }

    pub fn default_column(&self) -> &Column {
        self.columns
            .get(&self.default_column)
            .expect("Default column not found")
    }

    pub fn set_default_column(
        &mut self,
        id: ColumnId,
    ) -> Result<(), BoardError> {
        if !self.columns.contains_key(&id) {
            return Err(BoardError::ColumnNotFound);
        }
        self.default_column = id;
        Ok(())
    }

    pub fn remove_column(
        &mut self,
        id: ColumnId,
    ) -> Result<(), BoardError> {
        if id == self.default_column {
            return Err(
                BoardError::DefaultColumnCannotBeDeleted
            );
        }
        let column = self.columns
            .get(&id)
            .ok_or(
                BoardError::ColumnNotFound
            )?;

        if !column.tasks().is_empty() {
            return Err(
                BoardError::ColumnContainsTasks
            );
        }

        self.columns.remove(&id);

        self.column_order.retain(
            |column_id| *column_id != id
        );

        Ok(())
    }

    pub(crate) fn task(
        &self,
        id: &TaskId
    ) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn create_task(
        draft: TaskDraft
    ) -> Task {
        Task::new(
            TaskId::new(),
            draft.title,
            draft.problem_url,
            draft.difficulty,
            draft.project_path,
            draft.notes,
        )
    }

    pub fn add_task(
        &mut self,
        draft: TaskDraft,
    ) -> Result<(), BoardError> {

        let task = Self::create_task(draft);
        let id = task.id();

        self.tasks.insert(id, task);

        self.columns
            .get_mut(&self.default_column)
            .ok_or(BoardError::ColumnNotFound)?
            .add_task(id);

        Ok(())
    }
}