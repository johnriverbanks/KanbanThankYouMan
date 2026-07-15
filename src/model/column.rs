use crate::assets::colour::Colour;
use crate::model::column_error::ColumnError;
use crate::model::column_id::ColumnId;
use crate::model::task_draft::TaskDraft;
use crate::model::task_id::TaskId;
use crate::model::tasks::Task;

pub struct Column {
    id: ColumnId,
    name: String,
    colour: Colour,
    tasks: Vec<TaskId>,
}

impl Column {

    pub fn id(&self) -> &ColumnId {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }

    pub fn tasks(&self) -> &[TaskId] {
        self.tasks.as_slice()
    }

    pub fn new(
        id: ColumnId,
        name: String,
        colour: Colour
    ) -> Self {
        Self {
            id,
            name,
            colour,
            tasks: Vec::new(),
        }
    }

    pub fn create(
        name: &str,
        colour: Colour,
    ) -> Self {
        Self{
            id: ColumnId::new(),
            name: name.to_string(),
            colour,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(
        &mut self, task_id: TaskId
    ) {
        self.tasks.push(task_id);
    }

    pub fn remove_task(
        &mut self,
        task_id: &TaskId,
    ) -> Result<(), ColumnError> {
        let index = self.tasks
            .iter()
            .position(|id| id == task_id)
            .ok_or(ColumnError::TaskNotFound)?;
        self.tasks.remove(index);
        Ok(())
    }
}