use serde::{Deserialize, Serialize};
use crate::model::difficulty::Difficulty;
use crate::model::task_draft::TaskDraft;
use crate::model::task_id::TaskId;

#[derive(
    Serialize,
    Deserialize
)]

pub struct Task {
    id: TaskId,
    title: String,
    problem_url: String,
    difficulty: Difficulty,
    project_path: String,
    notes: String,
}

impl Task {

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn problem_url(&self) -> &str {
        self.problem_url.as_str()
    }

    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn project_path(&self) -> &str {
        self.project_path.as_str()
    }

    pub fn notes(&self) -> &str {
        self.notes.as_str()
    }

    pub (crate) fn new(
        id: TaskId,
        title: String,
        problem_url: String,
        difficulty: Difficulty,
        project_path: String,
        notes: String,
    ) -> Self {
        Self {
            id,
            title,
            problem_url,
            difficulty,
            project_path,
            notes,
        }
    }

    pub fn update(
        &mut self,
        draft: TaskDraft,
    ) {
        self.title = draft.title;
        self.problem_url = draft.problem_url;
        self.difficulty = draft.difficulty;
        self.project_path = draft.project_path;
        self.notes = draft.notes;
    }
}