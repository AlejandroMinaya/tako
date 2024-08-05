use egui::{
    Context,
    ViewportBuilder,
    Ui,
    CentralPanel,
    TopBottomPanel,
    Color32,
    Vec2,
    Button,
    Window,
    Key,
    text::LayoutJob,
    FontId,
    FontFamily
};
use eframe::{
    NativeOptions,
    Storage,
    run_native
};
use crate::core::tasks::{Oswald, Task};

const TASK_BG: Color32 = Color32::from_rgb(109, 33, 79);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const TASK_RADIUS: f32 = 8.0;
const TASK_SIZE: Vec2 = Vec2 { x: 160.0, y: 98.89 };
const TASK_FONT_SIZE: f32 = 16.0;
const TASK_INNER_MARGIN: f32 = 32.0;
const BOTTOM_PANEL_HEIGHT: f32 = 8.0;

impl Task {
    fn ui(&self, ui: &mut Ui) {
        let task_desc = LayoutJob::simple(
            self.desc.clone(),
            FontId {
                size: TASK_FONT_SIZE, family: FontFamily::Proportional
            },
            TASK_FG,
            TASK_SIZE.x - TASK_INNER_MARGIN
        );
        let button = Button::new(task_desc)
            .fill(TASK_BG)
            .frame(false)
            .rounding(TASK_RADIUS)
            .min_size(TASK_SIZE)
            .sense(egui::Sense::drag());
        ui.add(button);
    }
    fn form_ui(&mut self, ui: &mut Ui) {
        ui.label("Task:");
        ui.text_edit_singleline(&mut self.desc);
    }
}

struct Tako {
    oswald: Oswald,
    task_form_open: bool,
    form_task: Task,
    next_id: u32
}
impl Tako {
    fn save_form_task(&mut self) {
        let task = self.form_task.clone();
        self.oswald.add_task(Box::new(task));
    }
}
impl eframe::App for Tako {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for task in self.oswald.get_all_tasks() {
                    self.next_id = std::cmp::max(self.next_id, task.id + 1);
                    task.ui(ui);
                }
            });
        });

        TopBottomPanel::bottom("menu")
            .resizable(false)
            .min_height(BOTTOM_PANEL_HEIGHT)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("New Task").clicked() {
                        self.task_form_open = !self.task_form_open;
                        self.form_task.id = self.next_id;
                    }
                });
            });


        let task_form_window = Window::new("Task")
            .title_bar(false)
            .auto_sized();

        if self.task_form_open {
            task_form_window.show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.form_task.form_ui(ui);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() { 
                            self.save_form_task();
                            self.task_form_open = false;
                        }
                        if ui.button("Cancel").clicked() { 
                            self.task_form_open = false;
                        }
                    });
                });
                if ctx.input(|i| i.key_released(Key::Escape)) {
                    self.task_form_open = false;
                }
            });
        }

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
        Ok(Box::new(Tako { 
            oswald,
            form_task: Task::default(),
            task_form_open: false,
            next_id: 1
        }))
    }))
}
