use std::cmp::max;
use egui::{
    Window,
    Button,
    Context,
    ViewportBuilder,
    Color32,
    Vec2,
    Pos2,
    Ui,
    Frame,
    SidePanel,
    CentralPanel,
    Align,
    Align2,
    Response,
    Sense,
    FontId,
    ScrollArea,
    FontFamily,
    Rect,
    Area,
    Id,
    CursorIcon,
    text::LayoutJob
};
use eframe::{
    NativeOptions,
    Storage,
    run_native
};
use crate::core::tasks::{Oswald, Task, BoxTaskVec};

const INNER_MARGIN: f32 = 0.0;
const MENU_WIDTH: f32 = 144.0;

const BUTTON_SELECTED_BG: Color32 = Color32::from_rgb(119, 140, 163);
const BUTTON_HOVERED_BG: Color32 = Color32::from_rgb(165, 177, 194);
const BUTTON_BG: Color32 = Color32::from_rgb(47, 53, 66);
const BUTTON_FG: Color32 = Color32::from_rgb(241, 242, 246);
const BUTTON_FONT_SIZE: f32 = 12.0;
const BUTTON_MARGIN: f32 = 2.0;
const BUTTON_PADDING: f32 = 8.0;
const BUTTON_RADIUS: f32 = 16.0;

const TASK_BG: Color32 = Color32::from_rgb(109, 33, 79);
const TASK_HOVERED_BG: Color32 = Color32::from_rgb(179, 55, 113);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const TASK_FONT_SIZE: f32 = 12.0;
const TASK_MARGIN: f32 = 2.0;
const TASK_PADDING: f32 = 16.0;
const TASK_RADIUS: f32 = 8.0;
const TASK_SIZE: Vec2 = Vec2 { x: 120.0, y: 80.0 };

const DRAG_SPEED: f32 = 15.0;

fn norm_value(mut curr: f32, mut min_val: f32, mut max_val: f32) -> f32 {
    if max_val == min_val {
        return 0.0;
    }
    if min_val < 0.0 {
        curr += min_val.abs();
        max_val += min_val.abs();
        min_val = 0.0;
    }
    return (curr - min_val) / (max_val - min_val);
}

impl Task { 
    fn delta_update(&mut self, delta: &Vec2, area: &Rect) {
        let urgency_delta = delta.x / area.width() * DRAG_SPEED;
        let importance_delta = -delta.y / area.height() * DRAG_SPEED;
        dbg!(delta, urgency_delta, importance_delta);
        self.urgency += urgency_delta;
        self.importance += importance_delta;
    }
    fn get_arrange_rect(&self, stats: &Stats, area: &Rect) -> Rect {
        let norm_importance =  norm_value(self.importance, stats.min_importance, stats.max_importance);
        let norm_urgency =  norm_value(self.urgency, stats.min_urgency, stats.max_urgency);

        let half_task_width = TASK_SIZE.x/2.0;
        let half_task_height = TASK_SIZE.y/2.0;

        let area = Rect {
            min: Pos2 {
                x: area.min.x + half_task_width,
                y: area.min.y + half_task_height
            },
            max: Pos2 {
                x: area.max.x - half_task_width,
                y: area.max.y - half_task_height
            }
        };
        let area_width = area.width();
        let area_height = area.height();

        let center = Pos2 {
            x: area.min.x + norm_urgency * area_width,
            y: area.min.y + (1.0 - norm_importance) * area_height
        };
        let top_left = Pos2 {
            x: center.x - half_task_width,
            y: center.y - half_task_height
        };
        let bottom_right = Pos2 {
            x: center.x + half_task_width,
            y: center.y + half_task_height
        };
        Rect { min: top_left, max: bottom_right }
    }
    fn show_arrange(&self, ui: &mut Ui, stats: &Stats, area: &Rect) -> Response {
        let task_rect = self.get_arrange_rect(stats, area);
        let _ = ui.allocate_rect(task_rect, Sense::click_and_drag());
        let id = Id::new(format!("task_{}", self.id));
        let response = ui.interact(task_rect, id, Sense::click_and_drag());
        let mut background_color = 
            if response.hovered() {
                TASK_HOVERED_BG
            } else {
                TASK_BG
            };
        let complexity: f32 = self.get_complexity() as f32;
        if complexity > 1.0 {
            background_color = background_color.gamma_multiply(1.0/complexity
            );
        }
        let content_rect = task_rect.shrink(TASK_PADDING);
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
        ui.painter().rect_filled(task_rect, TASK_RADIUS, background_color); 
        ui.painter().galley(text_pos, text_galley, TASK_FG);

        if response.hovered () {
            ui.ctx().set_cursor_icon(CursorIcon::Grab);
        }
        if response.dragged() {
            ui.ctx().set_cursor_icon(CursorIcon::Move);
        }

        response
    }

    fn show_overview(&mut self, ui: &mut Ui) -> Response {
        let (task_rect, response) = ui.allocate_at_least(TASK_SIZE, Sense::click());
        let mut background_color = 
            if response.hovered() {
                TASK_HOVERED_BG
            } else {
                TASK_BG
            };
        let complexity: f32 = self.get_complexity() as f32;
        if complexity > 1.0 {
            background_color = background_color.gamma_multiply(1.0/complexity
            );
        }
        let content_rect = task_rect.shrink(TASK_PADDING);
        let mut text_layout = LayoutJob::simple(
            self.desc.clone(),
            FontId { size: TASK_FONT_SIZE, family: FontFamily::Monospace },
            TASK_FG,
            content_rect.width()
        );
        text_layout.halign = Align::Center;
        let mut text_pos = Align2::CENTER_CENTER.pos_in_rect(&content_rect);
        text_pos.y -= TASK_FONT_SIZE/2.0;

        Frame::default()
            .outer_margin(TASK_MARGIN)
            .show(ui, |ui| {
                let text_galley = ui.painter().layout_job(text_layout);
                ui.painter().rect_filled(task_rect, TASK_RADIUS, background_color); 
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

#[derive(Debug)]
struct Stats {
    max_urgency: f32,
    min_urgency: f32,
    max_importance: f32,
    min_importance: f32
}
impl Stats {
    fn from_tasks(tasks: &Vec<&Task>) -> Stats {
        let mut stats = Stats {
            max_importance: 0.0,
            min_importance: f32::MAX,
            max_urgency: 0.0,
            min_urgency: f32::MAX,
        };

        for task in tasks {
            stats.max_importance = f32::max(stats.max_importance, task.importance);
            stats.min_importance = f32::min(stats.min_importance, task.importance);
            stats.max_urgency = f32::max(stats.max_urgency, task.urgency);
            stats.min_urgency = f32::min(stats.min_urgency, task.urgency);
        }

        stats
    }
}
struct Tako {
    oswald: Oswald,
    current_view: View,
    target_daily_tasks: usize,
    overview_columns: usize,
    form_task: Option<Task>,
    arrange_parent_task: Option<Task>,
    arrange_prev_parents: Vec<Task>,
    next_task_id: u32
}
impl Tako {
    fn tako_full_button(&self, ui: &mut Ui, text: &str, selected: bool) -> Response {
        let width = ui.available_width();
        let height = BUTTON_FONT_SIZE + BUTTON_PADDING;
        let (rect, response) = ui.allocate_exact_size(
            [width, height].into(), Sense::click()
        );
        let background_color = 
            if selected {
                BUTTON_SELECTED_BG
            } else if response.hovered() {
                BUTTON_HOVERED_BG
            } else {
                BUTTON_BG
            };
        let content_rect = rect.shrink(BUTTON_PADDING);
        let mut text_layout = LayoutJob::simple(
            text.to_string(),
            FontId { size: BUTTON_FONT_SIZE, family: FontFamily::Monospace },
            BUTTON_FG,
            content_rect.width()
        );
        text_layout.halign = Align::Center;
        let mut text_pos = Align2::CENTER_CENTER.pos_in_rect(&content_rect);
        text_pos.y -= BUTTON_FONT_SIZE/2.0;

        Frame::default()
            .outer_margin(BUTTON_MARGIN)
            .show(ui, |ui| {
                let text_galley = ui.painter().layout_job(text_layout);
                ui.painter().rect_filled(rect, BUTTON_RADIUS, background_color);
                ui.painter().galley(text_pos, text_galley, BUTTON_FG);
            });

        response
    }

    fn show_menu(&mut self, ctx: &Context) {
        SidePanel::left("Options")
            .exact_width(MENU_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("tako");
                    ui.spacing();
                    if self.tako_full_button(ui, "Overview", matches!(self.current_view, View::Overview)).clicked() {
                        self.current_view = View::Overview;
                    }
                    if self.tako_full_button(ui, "Arrange", matches!(self.current_view, View::Arrange)).clicked() {
                        self.current_view = View::Arrange;
                    }
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
    fn show_arrange_frame(&mut self, ui: &mut Ui, ctx: &Context) {
        self.show_task_form(ctx);
        Frame::default()
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.add_sized(Vec2::new(144.0, 16.0), Button::new("Add Task")).clicked() {
                            self.form_task = Some(Task::new_with_id(self.next_task_id));
                        }
                        ui.horizontal(|ui| {
                            if self.arrange_parent_task.is_some() {
                                if ui.button("Home").clicked() {
                                    self.arrange_prev_parents.clear();
                                    self.arrange_parent_task = None;
                                };
                                if ui.button("Back").clicked() {
                                    self.arrange_parent_task = self.arrange_prev_parents.pop()
                                }
                            }
                        });
                    });
                    ui.separator();
                    let (_, area_rect) = ui.allocate_space(ui.available_size());
                    Area::new("Arrange".into())
                        .movable(true)
                        .default_size(ui.available_size())
                        .constrain_to(area_rect)
                        .show(ctx, |ui| {
                            let tasks = match &self.arrange_parent_task {
                                Some(parent_task) => parent_task.get_subtasks(),
                                None => self.oswald.get_tasks()
                            };
                            let mut updated_tasks: BoxTaskVec = vec![];
                            let mut new_parent_task: Option<Task> = None;
                            let task_stats = Stats::from_tasks(&tasks);

                            for task in tasks {
                                let response = task.show_arrange(ui, &task_stats, &area_rect);

                                if response.triple_clicked() {
                                    self.form_task = Some(task.clone());
                                } else if response.double_clicked() {
                                    new_parent_task = Some(task.clone());
                                }


                                if response.dragged() {
                                    let delta = response.drag_motion();
                                    if delta != Vec2::ZERO {
                                        let mut task = task.clone();
                                        task.delta_update(&delta, &area_rect);
                                        updated_tasks.push(Box::new(task));
                                    }
                                }
                            }

                            if let Some(new_parent) = new_parent_task.take() {
                                if let Some(old_parent) = self.arrange_parent_task.take() {
                                    self.arrange_prev_parents.push(old_parent);
                                }
                                self.arrange_parent_task = Some(new_parent);
                            }

                            updated_tasks.into_iter().for_each(|task| {
                                self.oswald.add_task(task);
                            });
                        });
                });
            });
    }
    fn show_task_form(&mut self, ctx: &Context) {
        let mut pending_cancel = false;
        let mut pending_save = false;
        if let Some(task) = &mut self.form_task {
            Window::new("Task Form")
                .title_bar(false)
                .show(ctx, |ui| { 
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut task.desc);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            pending_cancel = true;
                        } 
                        if ui.button("Save").clicked() {
                            pending_save = true;
                        }
                    });
                });
            });
        }
        if pending_cancel { self.form_task = None; }
        if pending_save {
            if let Some(task) = self.form_task.take() {
                let task = Box::new(task);

                if task.id == self.next_task_id {
                    self.next_task_id += 1;
                }

                match self.arrange_parent_task.take() {
                    Some(mut parent_task) => {
                        parent_task.add_subtask(task);
                        self.arrange_parent_task = Some(parent_task.clone());
                        self.oswald.add_task(Box::new(parent_task));
                    },
                    None => {
                        self.oswald.add_task(task);
                    }
                }
            }
        }
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
        self.show_menu(ctx);

        CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Overview => self.show_overview_frame(ui),
                View::Arrange => self.show_arrange_frame(ui, ctx)
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
        let mut next_task_id: u32 = 1;

        for task in oswald.get_all_tasks() {
            next_task_id = max(next_task_id, task.id + 1);
        }
        Ok(Box::new(Tako {
            oswald, 
            form_task: None,
            arrange_parent_task: None,
            arrange_prev_parents: vec![],
            current_view: View::Overview,
            target_daily_tasks: 5,
            overview_columns: 3,
            next_task_id,
        }))
    }))
}
