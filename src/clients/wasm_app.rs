use egui::{
    Context,
    ViewportBuilder,
    Ui,
    CentralPanel,
    TopBottomPanel,
    Color32,
    Vec2,
    Window,
    Align,
    Align2,
    Key,
    CursorIcon,
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
const SELECTED_TASK_BG: Color32 = Color32::from_rgb(106, 176, 76);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const SELECTED_TASK_FG: Color32 = Color32::from_rgb(248, 239, 186);
const TASK_RADIUS: f32 = 8.0;
const TASK_SIZE: Vec2 = Vec2 { x: 160.0, y: 98.89 };
const TASK_FONT_SIZE: f32 = 12.0;
const TASK_INNER_MARGIN: f32 = 16.0;
const BOTTOM_PANEL_HEIGHT: f32 = 8.0;

impl Task {
    fn ui(&self, app: &Tako, ui: &mut Ui) -> (egui::Id, egui::Rect) {
        let mut bg = TASK_BG;
        let mut fg = TASK_FG;

        if let Some(selected) = app.selected_task.as_ref() {
            if (selected.id == self.id) {
                bg = SELECTED_TASK_BG; 
                fg = SELECTED_TASK_FG;
            }
        }

        let (id, rect) = ui.allocate_space(TASK_SIZE);
        ui.painter().rect_filled(rect, TASK_RADIUS, bg);

        let content_rect = rect.shrink(TASK_INNER_MARGIN);

        let mut text = LayoutJob::simple(
            self.desc.clone(),
            FontId { size: TASK_FONT_SIZE, family: FontFamily::Monospace },
            fg,
            content_rect.width()
        );
        text.halign = Align::Center;
        let mut text_pos = Align2::CENTER_CENTER.pos_in_rect(&content_rect);
        text_pos.y -= TASK_FONT_SIZE/2.0;
        let galley = ui.painter().layout_job(text);
        ui.painter().galley(text_pos, galley, TASK_FG);

        return (id, rect)

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
    pub selected_task: Option<Task>,
    next_id: u32
}
impl Tako {
    fn save_form_task(&mut self) {
        let task = self.form_task.clone();
        if let Some(mut selected) = self.selected_task.take() {
            selected.add_subtask(Box::new(task));
            self.oswald.add_task(Box::new(selected));
        } else {
            self.oswald.add_task(Box::new(task));
        }
    }
}
impl eframe::App for Tako {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for task in self.oswald.get_all_tasks() {
                    self.next_id = std::cmp::max(self.next_id, task.id + 1);
                    let (id, rect) = task.ui(&self, ui);
                    if ui.interact(rect, id, egui::Sense::click()).clicked {
                        self.selected_task = Some(task.clone());
                    }
                }
            });
        });

        TopBottomPanel::bottom("menu")
            .resizable(false)
            .min_height(BOTTOM_PANEL_HEIGHT)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    if ui.button("New Task").clicked() {
                        self.task_form_open = !self.task_form_open;
                        self.form_task = Task::new_with_id(self.next_id);
                        self.selected_task = None;
                    }
                    
                    if self.selected_task.is_some() {
                        if ui.button("Add Subtask").clicked() {
                            self.task_form_open = !self.task_form_open;
                            self.form_task = Task::new_with_id(self.next_id);
                        }
                        if ui.button("Edit Task").clicked() {
                            if let Some(selected) = self.selected_task.take() {
                                self.task_form_open = !self.task_form_open;
                                self.form_task = selected;

                            }
                        }
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
            selected_task: None,
            task_form_open: false,
            next_id: 1
        }))
    }))
}
