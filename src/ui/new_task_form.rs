use crate::model::board::Board;
use crate::ui::window_status::WindowStatus;
use crate::model::difficulty::Difficulty;
use crate::model::task_draft::TaskDraft;

pub struct NewTaskForm {
    title: String,
    problem_url: String,
    difficulty: Difficulty,
    project_path: String,
    notes: String,
}

impl NewTaskForm {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        board: &mut Board,
    ) -> WindowStatus {
        let mut window_status = WindowStatus::Open;

        egui::Window::new("New Task")
            .show(ctx, |ui| {
                egui::Grid::new("new_task_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Title");
                        ui.text_edit_singleline(&mut self.title);
                        ui.end_row();

                        ui.label("Problem URL");
                        ui.text_edit_singleline(&mut self.problem_url);
                        ui.end_row();

                        ui.label("Difficulty");
                        egui::ComboBox::from_label("")
                            .selected_text(self.difficulty.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.difficulty,
                                    Difficulty::Unknown,
                                    "Unknown",
                                );
                                ui.selectable_value(
                                    &mut self.difficulty,
                                    Difficulty::Easy,
                                    "Easy",
                                );
                                ui.selectable_value(
                                    &mut self.difficulty,
                                    Difficulty::Medium,
                                    "Medium",
                                );
                                ui.selectable_value(
                                    &mut self.difficulty,
                                    Difficulty::Hard,
                                    "Hard",
                                );
                            });
                        ui.end_row();

                        ui.label("Project Path");
                        ui.text_edit_singleline(&mut self.project_path);
                        ui.end_row();

                        ui.label("Notes");
                        ui.text_edit_singleline(&mut self.notes);
                        ui.end_row();
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.cancel();
                        window_status = WindowStatus::Close;
                    }

                    if ui.button("Create").clicked() {
                        self.submit_task(board);
                        window_status = WindowStatus::Close;
                    }

                    if ui.button("Create More").clicked() {
                        self.submit_task(board);
                    }
                })
            });
        window_status
    }

    pub fn new() -> Self {
        Self {
            title: String::new(),
            problem_url: String::new(),
            difficulty: Difficulty::Unknown,
            project_path: String::new(),
            notes: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.title.clear();
        self.problem_url.clear();
        self.difficulty = Difficulty::Unknown;
        self.project_path.clear();
        self.notes.clear();
    }

    fn cancel(&mut self) {
        self.clear();
    }

    fn submit_task(&mut self, board: &mut Board,) {
        if let Err(err) = board.add_task(self.build_task_draft()) {
            eprintln!("Failed to create task: {:?}", err);
        }
    }

    fn build_task_draft(&mut self) -> TaskDraft {
        let difficulty = std::mem::replace(
            &mut self.difficulty,
            Difficulty::Unknown,
        );
        TaskDraft {
            title: std::mem::take(&mut self.title),
            problem_url: std::mem::take(&mut self.problem_url),
            difficulty,
            project_path: std::mem::take(&mut self.project_path),
            notes: std::mem::take(&mut self.notes),
        }
    }
}