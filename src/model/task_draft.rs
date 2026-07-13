use crate::model::difficulty::Difficulty;

pub struct TaskDraft {
    pub title: String,
    pub problem_url: String,
    pub difficulty: Difficulty,
    pub project_path: String,
    pub notes: String,
}