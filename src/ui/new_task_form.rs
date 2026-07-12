use crate::model::board::Board;
use crate::ui::window_status::WindowStatus;
use crate::model::difficulty::Difficulty;
use crate::model::task::Task;

pub struct NewTaskForm {
    title: String,
    problem_url: String,
    difficulty: Difficulty,
    project_path: String,
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
                });

                ui.separator();

                ui.horizontal(|ui| {

                if ui.button("Cancel").clicked() {
                    window_status = self.cancel();
                }

                if ui.button("Create").clicked() {
                    window_status =self.create(board);
                }

                if ui.button("Create More").clicked() {
                    window_status = self.create_more(board);
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
        }
    }

    pub fn clear(&mut self) {
        self.title.clear();
        self.problem_url.clear();
        self.difficulty = Difficulty::Unknown;
        self.project_path.clear();
    }

    fn cancel(&mut self) -> WindowStatus {
        self.clear();
        WindowStatus::Close
    }

    fn create(&mut self, board: &mut Board)-> WindowStatus {
        self.create_task(board);
        WindowStatus::Close
    }

    fn create_more(&mut self, board: &mut Board) -> WindowStatus {
        self.create_task(board);
        WindowStatus::Open
    }

    fn create_task(&mut self, board: &mut Board) {
        let difficulty = std::mem::replace(
            &mut self.difficulty,
            Difficulty::Unknown,
        );
        let task = Task::new(
            &self.title,
            &self.problem_url,
            difficulty,
            &self.project_path,
        );
        board.add_task(task);
        self.clear();
    }
}