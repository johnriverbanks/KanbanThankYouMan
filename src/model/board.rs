use crate::assets::colour::Colour;
use crate::model::column::Column;
use crate::model::task::Task;

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

    pub fn add_task(&mut self, task: Task) {
            self.columns[2].add_task(task);
    }

}