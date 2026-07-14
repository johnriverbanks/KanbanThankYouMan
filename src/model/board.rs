use std::collections::HashMap;
use crate::model::column::Column;
use crate::model::column_id::ColumnId;
use crate::model::tasks::Task;
use crate::model::task_draft::TaskDraft;
use crate::model::task_id::TaskId;
use crate::storage::board_loader::BoardLoader;

pub struct Board {
    columns: HashMap<ColumnId, Column>,
    column_order: Vec<ColumnId>,
    tasks: HashMap<TaskId, Task>,
}

impl Board {
    
    pub(crate) fn empty() -> Self {
        Self {
            columns: HashMap::new(),
            column_order: Vec::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        BoardLoader::load()
    }

    pub fn add_column(
        &mut self,
        column: Column,
    ) {
        let id = column.id().clone();
        self.column_order.push(
            id.clone()
        );
        self.columns.insert(
            id,
            column
        );
    }

    pub fn columns(&self) -> impl Iterator<Item = &Column> {
        self.column_order
            .iter()
            .filter_map(
                |id| self.columns.get(id)
            )
    }

    pub(crate) fn task(&self, id: &TaskId) -> Option<&Task> {
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
    )
    {
        let task = Task::new(
            TaskId::new(),
            draft.title,
            draft.problem_url,
            draft.difficulty,
            draft.project_path,
            draft.notes,
        );
            let id = task.id();
        self.tasks.insert(id, task);
        self.columns
            .get_mut(&ColumnId::from_static("unresolved"))
            .expect("column not found")
            .add_task(id);
    }
}