mod app;
mod assets;
mod commands;
mod model;
mod storage;
mod ui;

use app::KanbanThankYouMan;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "KanbanThankYouMan",
        options,
        Box::new(|cc| Ok(Box::new(KanbanThankYouMan::new(cc)))),
    )
}