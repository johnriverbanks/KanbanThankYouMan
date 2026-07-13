use crate::model::difficulty::Difficulty;

pub struct Task {
    title: String,
    problem_url: String,
    difficulty: Difficulty,
    project_path: String,
}

impl Task {
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

    pub fn new(
        title: String,
        problem_url: String,
        difficulty: Difficulty,
        project_path: String,
    ) -> Self {
        Self {
            title,
            problem_url,
            difficulty,
            project_path,
        }
    }
}