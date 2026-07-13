use crate::assets::colour::Colour;
use crate::model::column::Column;
use crate::model::task::Task;
use crate::model::task_draft::TaskDraft;
use crate::model::task_id::TaskId;

pub struct Board {
    columns: Vec<Column>,
}

impl Board {
    pub fn columns(&self) -> &[Column] {
        &self.columns.as_slice()
    }

    pub(crate) fn new() -> Self {
        Self {
            columns: vec![
                Column::new("Today", Colour::Blue),
                Column::new("Unresolved", Colour::Red),
                Column::new("Needs Review", Colour::Amber),
                Column::new("Solved!", Colour::Green),
            ],
        }
    }

    fn create_task(&self, draft: TaskDraft) -> Task {
        Task::new(
            TaskId::new(),
            draft.title,
            draft.problem_url,
            draft.difficulty,
            draft.project_path,
            draft.notes,
        )
    }

    pub fn add_task(&mut self, draft: TaskDraft) {
        let task = self.create_task(draft);
        self.columns[1].add_task(task);
    }
}