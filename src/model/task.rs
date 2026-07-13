use crate::model::difficulty::Difficulty;
use crate::model::task_id::TaskId;

pub struct Task {
    id: TaskId,
    title: String,
    problem_url: String,
    difficulty: Difficulty,
    project_path: String,
    notes: String,
}

impl Task {

    pub fn id(&self) -> &TaskId {&self.id}

    pub fn title(&self) -> &str {self.title.as_str()}

    pub fn problem_url(&self) -> &str {self.problem_url.as_str()}

    pub fn difficulty(&self) -> &Difficulty {&self.difficulty}

    pub fn project_path(&self) -> &str {self.project_path.as_str()}

    pub fn notes(&self) -> &str {self.notes.as_str()}

    pub fn new(
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
}