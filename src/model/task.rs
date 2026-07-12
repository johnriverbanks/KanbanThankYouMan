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
        title: &str,
        problem_url: &str,
        difficulty: Difficulty,
        project_path: &str,
    ) -> Self {
        Self {
            title: title.to_string(),
            problem_url: problem_url.to_string(),
            difficulty,
            project_path: project_path.to_string(),
        }
    }
}

