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
    task_locations: HashMap<TaskId, ColumnId>,
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
            task_locations: HashMap::new(),
            default_column,
        })
    }

    pub fn add_column(
        &mut self,
        column: Column,
    ) -> Result<(), BoardError> {
        //Error Here
        let id = column.id();
        if self.columns.contains_key(&id) {
            return Err(BoardError::DuplicateColumnId)
        }
        self.column_order.push(id);
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

        self.task_locations
            .insert(id, self.default_column);

        Ok(())
    }

    fn column_of_task(
        &self,
        task_id: &TaskId,
    ) -> Option<ColumnId> {
        self.task_locations
            .get(task_id)
            .copied()
    }

    fn move_task_between_columns(
        &mut self,
        task_id: TaskId,
        source_id: ColumnId,
        destination_id: ColumnId,
        index: usize,
    ) -> Result<(), BoardError> {
        {
            let source = self.columns
                .get_mut(&source_id)
                .ok_or(BoardError::ColumnNotFound)?;
            source.remove_task(&task_id)?;
        }
        {
            let destination = self.columns
                .get_mut(&destination_id)
                .ok_or(BoardError::ColumnNotFound)?;
            destination.insert_task(index, task_id)?;
        }
        self.task_locations
            .insert(task_id, destination_id);
        Ok(())
    }

    fn reorder_task(
        &mut self,
        task_id: TaskId,
        column_id: ColumnId,
        index: usize,
    ) -> Result<(), BoardError> {

        let column = self.columns
            .get_mut(&column_id)
            .ok_or(BoardError::ColumnNotFound)?;

        let current_index = column
            .position_of(&task_id)
            .ok_or(BoardError::ColumnNotFound)?;

        column.remove_task(&task_id)?;

        let adjusted_index =
            if index > current_index {
                index - 1
            } else {
                index
            };

        column.insert_task(adjusted_index, task_id)?;

        Ok(())
    }

    pub fn move_task(
        &mut self,
        task_id: TaskId,
        destination_id: ColumnId,
        index: usize,
        adjusted_index: usize,
    ) -> Result<(), BoardError> {

        if !self.tasks.contains_key(&task_id) {
            return Err(BoardError::TaskNotFound);
        }

        if !self.columns.contains_key(&destination_id) {
            return Err(BoardError::ColumnNotFound);
        }

        let source_id = self
            .column_of_task(&task_id)
            .ok_or(BoardError::TaskNotFound)?;

        if source_id == destination_id {
            self.reorder_task(
                task_id,
                source_id,
                adjusted_index,
            )
        } else {
            self.move_task_between_columns(
                task_id,
                source_id,
                destination_id,
                index,
            )
        }
    }

    pub fn remove_task(
        &mut self,
        task_id: TaskId,
    ) -> Result<Task, BoardError> {

        let column_id = self
            .column_of_task(&task_id)
            .ok_or(BoardError::TaskNotFound)?;

        let column = self.columns
            .get_mut(&column_id)
            .ok_or(BoardError::ColumnNotFound)?;
        column.remove_task(&task_id)?;

        self.task_locations.remove(&task_id);

        let task = self.tasks
            .remove(&task_id)
            .ok_or(BoardError::TaskNotFound)?;

        Ok(task)
    }

    pub fn edit_task(
        &mut self,
        task_id: TaskId,
        draft: TaskDraft,
    ) -> Result<(), BoardError> {
        let task = self.tasks
        .get_mut(&task_id)
        .ok_or(BoardError::TaskNotFound)?;

    task.update(draft);

    Ok(())
    }

}