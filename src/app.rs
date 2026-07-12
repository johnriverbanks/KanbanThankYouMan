use crate::model::board::Board;
use crate::ui::new_task_form::NewTaskForm;
use crate::ui::window_status::WindowStatus;

pub struct KanbanThankYouMan {
    board: Board,
    new_task_form: Option<NewTaskForm>,
}

impl KanbanThankYouMan {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: Board::new(),
            new_task_form: None,
        }
    }
}

impl eframe::App for KanbanThankYouMan {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ui, |ui| {

            ui.horizontal(|ui| {
                for column in self.board.columns() {
                    ui.group(|ui| {
                        ui.heading(column.name());

                        for task in column.task() {
                            ui.label(task.title());
                        }
                    });
                }
            });

            if ui.button("Add Task").clicked() {
                self.new_task_form = Some(NewTaskForm::new());
            }
        });

        let status = if let Some(form) = &mut self.new_task_form {
            Some(form.show(ui.ctx(), &mut self.board))
        } else {
            None
        };
        if matches!(status, Some(WindowStatus::Close)) {
            self.new_task_form = None;
        }
    }
}