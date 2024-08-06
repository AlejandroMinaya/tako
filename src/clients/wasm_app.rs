use std::cmp::max;
use egui::{
    Context,
    ViewportBuilder,
    Color32,
    Vec2,
    Ui,
    Frame,
    TopBottomPanel,
    CentralPanel,
    Layout,
    Align,
    Align2,
    Response,
    Sense,
    FontId,
    ScrollArea,
    FontFamily,
    text::LayoutJob
};
use eframe::{
    NativeOptions,
    Storage,
    run_native
};
use crate::core::tasks::{Oswald, Task};

const INNER_MARGIN: f32 = 0.0;
const TASK_BG: Color32 = Color32::from_rgb(109, 33, 79);
const SELECTED_TASK_BG: Color32 = Color32::from_rgb(106, 176, 76);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const SELECTED_TASK_FG: Color32 = Color32::from_rgb(248, 239, 186);
const TASK_RADIUS: f32 = 8.0;
const TASK_SIZE: Vec2 = Vec2 { x: 120.0, y: 80.0 };
const TASK_FONT_SIZE: f32 = 12.0;
const TASK_PADDING: f32 = 16.0;
const TASK_MARGIN: f32 = 2.0;
const BOTTOM_PANEL_HEIGHT: f32 = 8.0;

impl Task { 
    fn show_overview(&mut self, ui: &mut Ui) -> Response {
        let (task_rect, response) = ui.allocate_at_least(TASK_SIZE, Sense::click());

        let content_rect = task_rect.shrink(TASK_PADDING);

        Frame::default()
            .outer_margin(TASK_MARGIN)
            .show(ui, |ui| {
            ui.painter().rect_filled(task_rect, TASK_RADIUS, TASK_BG);
            let mut text_layout = LayoutJob::simple(
                self.desc.clone(),
                FontId { size: TASK_FONT_SIZE, family: FontFamily::Monospace },
                TASK_FG,
                content_rect.width()
            );
            text_layout.halign = Align::Center;
            let mut text_pos = Align2::CENTER_CENTER.pos_in_rect(&content_rect);
            text_pos.y -= TASK_FONT_SIZE/2.0;
            let text_galley = ui.painter().layout_job(text_layout);
            ui.painter().galley(text_pos, text_galley, TASK_FG);
        });

        response
    }
}

#[derive(Default)]
enum View {
    Arrange,
    #[default]
    Overview
}
struct Tako {
    oswald: Oswald,
    current_view: View,
    target_daily_tasks: usize,
    overview_columns: usize,
    next_task_id: u32
}
impl Tako {
    fn show_top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Arrange").clicked() {
                        self.current_view = View::Arrange;
                    }
                    if ui.button("Overview").clicked() {
                        self.current_view = View::Overview;
                    }
                });
            });
        });
    }
    fn show_overview_frame(&mut self, ui: &mut Ui) {
        Frame::default()
            .inner_margin(INNER_MARGIN)
            .show(ui, |ui| {
                let mut curr_column = self.overview_columns - 1;
                let enumerated_tasks = self.oswald.get_all_tasks().into_iter().enumerate();
                ScrollArea::vertical().show(ui, |ui| {
                    ui.columns(self.overview_columns, |columns| {
                        for (idx, task) in enumerated_tasks {
                            if idx > 0 && idx % self.target_daily_tasks == 0 { curr_column -= 1; }
                            let mut task = task.clone();
                            if let Some(column) = columns.get_mut(curr_column) {
                                task.show_overview(column);
                            }
                        }
                    });
                });
            });
    }
    fn show_arrange_frame(&mut self, ui: &mut Ui) {
        Frame::default()
            .fill(Color32::GREEN)
            .show(ui, |ui| {
                dbg!(ui.available_size());
            });
    }
}
impl eframe::App for Tako {
    fn save(&mut self, storage: &mut dyn Storage) {
        let tasks = self.oswald.get_tasks();
        match serde_json::to_string(&tasks) {
            Ok(tasks_str) => { storage.set_string("tasks", tasks_str) },
            Err(err) => { println!("Couldn't save tasks: {err}") }
        }
    }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) { 
        self.show_top_bar(ctx);

        CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Overview => self.show_overview_frame(ui),
                View::Arrange => self.show_arrange_frame(ui)
            }
        });
    }
}
pub async fn start(mut oswald: Oswald) -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default(),
        ..Default::default()
    };
    run_native("Tako", options, Box::new(|cc| {
        if let Some(storage) = cc.storage { 
            let tasks_str = storage.get_string("tasks")
                .unwrap_or("[]".to_owned());
            let raw_tasks: Vec<Task> = serde_json::from_str(&tasks_str)?;
            for task in raw_tasks { oswald.add_task(Box::new(task)); }
        }
        Ok(Box::new(Tako {
            oswald, 
            current_view: View::Overview,
            target_daily_tasks: 2,
            overview_columns: 3,
            next_task_id: 1
        }))
    }))
}
