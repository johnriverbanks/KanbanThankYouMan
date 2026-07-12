use crate::assets::colour::Colour;
use crate::model::task::Task;

pub struct Column {
    name: String,
    colour: Colour,
    task: Vec<Task>,
}

impl Column {

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }

    pub fn task(&self) -> &[Task] {
        self.task.as_slice()
    }

    pub fn new(name: &str, colour: Colour) -> Self {
        Self {
            name: name.to_string(),
            colour,
            task: vec![],
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.task.push(task);
    }

}