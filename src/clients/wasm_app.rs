use egui::{
    Context,
    ViewportBuilder,
    Ui,
    CentralPanel,
    Color32,
    Vec2,
    Button,
    RichText
};
use eframe::{
    NativeOptions,
    Storage,
    run_native
};
use crate::core::tasks::{Oswald, Task};

const TASK_BG: Color32 = Color32::from_rgb(109, 33, 79);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const TASK_SIZE: Vec2 = Vec2 { x: 240.0, y: 148.33 };
const TASK_FONT_SIZE: f32 = 16.0;
const TASK_INNER_MARGIN: f32 = 32.0;

impl Task {
    fn ui(&self, ui: &mut Ui) {
        let text = RichText::new(&self.desc)
            .color(TASK_FG)
            .strong()
            .size(TASK_FONT_SIZE);
        let button = Button::new(text)
            .min_size(TASK_SIZE)
            .fill(TASK_BG)
            .frame(false)
            .sense(egui::Sense::drag());
        ui.add(button);
    }
}
struct Tako {
    oswald: Oswald
}
impl eframe::App for Tako {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for task in self.oswald.get_all_tasks() {
                    task.ui(ui)
                }
            });
        });
    }
    fn save(&mut self, storage: &mut dyn Storage) {
        let tasks = self.oswald.get_tasks();
        match serde_json::to_string(&tasks) {
            Ok(tasks_str) => { storage.set_string("tasks", tasks_str) },
            Err(err) => { println!("Couldn't save tasks: {err}") }
        }
    }
}
pub async fn start(mut oswald: Oswald) -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
        ,..Default::default()
    };
    run_native("Tako", options, Box::new(|cc| {
        if let Some(storage) = cc.storage { 
            let tasks_str = storage.get_string("tasks").unwrap_or("[]".to_owned());
            let raw_tasks: Vec<Task> = serde_json::from_str(&tasks_str)?;
            for task in raw_tasks {
                oswald.add_task(Box::new(task));
            }
        }
        Ok(Box::new(Tako { oswald }))
    }))
}
