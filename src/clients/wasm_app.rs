use std::cmp::max;
use std::time::Duration;
use egui::{
    Layout,
    Direction,
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
    CursorIcon,
    Id,
    text::LayoutJob
};
use eframe::{
    NativeOptions,
    Storage,
    run_native
};
use crate::core::tasks::{Oswald, Task, BoxTaskVec, TaskStatus};

const AUTO_SAVE_INTERVAL: Duration = Duration::new(5, 0);

const MENU_WIDTH: f32 = 144.0;

const BUTTON_SELECTED_BG: Color32 = Color32::from_rgb(119, 140, 163);
const BUTTON_HOVERED_BG: Color32 = Color32::from_rgb(165, 177, 194);
const BUTTON_BG: Color32 = Color32::from_rgb(47, 53, 66);
const BUTTON_FG: Color32 = Color32::from_rgb(241, 242, 246);
const BUTTON_FONT_SIZE: f32 = 12.0;
const BUTTON_MARGIN: f32 = 2.0;
const BUTTON_PADDING: f32 = 8.0;
const BUTTON_RADIUS: f32 = 16.0;

const ARRANGE_LABEL_FONT: FontId = FontId { size: 12.0, family: FontFamily::Monospace };
const ARRANGE_FG: Color32 = Color32::from_rgb(125, 125, 125);

const TASK_BG: Color32 = Color32::from_rgb(109, 33, 79);
const TASK_HOVERED_BG: Color32 = Color32::from_rgb(179, 55, 113);
const TASK_FG: Color32 = Color32::from_rgb(255, 204, 204);
const TASK_FONT_SIZE: f32 = 12.0;
const TASK_SMALL_FONT_SIZE: f32 = 10.0;
const TASK_PADDING: f32 = 16.0;
const TASK_RADIUS: f32 = 8.0;
const TASK_SIZE: Vec2 = Vec2 { x: 120.0, y: 80.0 };

const DONE_TASK_BG: Color32 = Color32::from_rgb(106, 176, 76);
const DONE_TASK_HOVERED_BG: Color32 = Color32::from_rgb(163, 203, 56);
const DONE_TASK_FG: Color32 = Color32::WHITE;

const ARCHIVED_TASK_BG: Color32 = Color32::from_rgb(10, 61, 98);
const ARCHIVED_TASK_HOVERED_BG: Color32 = Color32::from_rgb(60, 99, 130);
const ARCHIVED_TASK_FG: Color32 = Color32::from_rgb(223, 249, 251);

const MIN_DRAG_DELTA: f32 = 1e-2;
const MAX_ARRANGE_RECT: f32 = 100.0;
const MIN_ARRANGE_RECT: f32 = -100.0;
const RANGE_ARRANGE_RECT: f32 = MAX_ARRANGE_RECT - MIN_ARRANGE_RECT;

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
        let mut urgency_delta = delta.x / area.width() * RANGE_ARRANGE_RECT;
        let mut importance_delta = -delta.y / area.height() * RANGE_ARRANGE_RECT;
        urgency_delta = urgency_delta.signum() * urgency_delta.abs().max(MIN_DRAG_DELTA);
        importance_delta = importance_delta.signum() * importance_delta.abs().max(MIN_DRAG_DELTA);
        self.urgency += urgency_delta;
        self.importance += importance_delta;
    }
    fn get_arrange_rect(&self, area: &Rect) -> Rect {
        let norm_importance =  norm_value(self.importance, MIN_ARRANGE_RECT, MAX_ARRANGE_RECT);
        let norm_urgency =  norm_value(self.urgency, MIN_ARRANGE_RECT, MAX_ARRANGE_RECT);

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
    fn show_arrange(&self, ui: &mut Ui, area: &Rect) -> Response {
        let task_rect = self.get_arrange_rect(area);
        let mut child_ui = ui.child_ui(task_rect, Layout::centered_and_justified(Direction::TopDown), None);
        child_ui.add(self)
    }

    fn show_overview(&self, ui: &mut Ui) -> Response {
        let (task_rect, _) = ui.allocate_at_least(TASK_SIZE, Sense::click());
        let mut child_ui = ui.child_ui(task_rect, Layout::centered_and_justified(Direction::TopDown), None);
        child_ui.add(self)
    }
}

impl egui::Widget for &Task {
    fn ui(self, ui: &mut Ui) -> Response {
        let (_, rect) = ui.allocate_space(ui.available_size());
        let id = Id::new(format!("task_{}", self.id));
        let response = ui.interact(rect, id, Sense::click_and_drag());
        let task_stat = (self.status, response.hovered);
        let mut background_color = match task_stat {
            (TaskStatus::Done, true) => DONE_TASK_HOVERED_BG,
            (TaskStatus::Done, false) => DONE_TASK_BG,
            (TaskStatus::Archived, true) => ARCHIVED_TASK_HOVERED_BG,
            (TaskStatus::Archived, false) => ARCHIVED_TASK_BG,
            (_, true) => TASK_HOVERED_BG,
            (_, false) => TASK_BG
        };
        let font_color = match task_stat {
            (TaskStatus::Done, _) => DONE_TASK_FG,
            (TaskStatus::Archived, _) => ARCHIVED_TASK_FG,
            _ => TASK_FG
        };
        let complexity = self.get_complexity();
        background_color = background_color.gamma_multiply(1.0/complexity as f32);
        let content_rect = rect.shrink(TASK_PADDING);

        ui.painter().rect_filled(rect, TASK_RADIUS, background_color);

        let mut content_width = content_rect.width();
        if complexity > 1 {
            let complexity_galley = ui.painter().layout_no_wrap(
                format!("{}", complexity - 1),
                FontId { size: TASK_SMALL_FONT_SIZE, family: FontFamily::Monospace },
                font_color,
            );
            content_width -= complexity_galley.rect.width();

            let mut complexity_anchor = Align2::RIGHT_CENTER.pos_in_rect(&content_rect);
            complexity_anchor.x -= complexity_galley.rect.width()/2.0;
            complexity_anchor.y -= complexity_galley.rect.height()/2.0;
            ui.painter().galley(complexity_anchor, complexity_galley, font_color);

        }
        let desc_galley = ui.painter().layout(
            self.desc.clone(),
            FontId { size: TASK_FONT_SIZE, family: FontFamily::Monospace },
            font_color,
            content_width
        );
        let y_desc_offset = (content_rect.height() - desc_galley.rect.height()) / 2.0;
        let desc_pos = Pos2::new(content_rect.min.x, content_rect.min.y + y_desc_offset.max(0.0));
        ui.painter().galley(desc_pos, desc_galley, background_color);

        response
    }
}

#[derive(Default)]
enum View {
    Arrange,
    ArrangeAll,
    #[default]
    Overview
}

#[derive(Debug)]
struct Stats {
    max_urgency: f32,
    min_urgency: f32,
    range_urgency: f32,
    max_importance: f32,
    min_importance: f32,
    range_importance: f32
}
impl Stats {
    fn from_tasks(tasks: &Vec<&Task>) -> Stats {
        let mut stats = Stats {
            max_importance: 0.0,
            min_importance: f32::MAX,
            range_importance: 0.0,
            max_urgency: 0.0,
            min_urgency: f32::MAX,
            range_urgency: 0.0,
        };

        for task in tasks {
            stats.max_importance = f32::max(stats.max_importance, task.importance);
            stats.min_importance = f32::min(stats.min_importance, task.importance);
            stats.max_urgency = f32::max(stats.max_urgency, task.urgency);
            stats.min_urgency = f32::min(stats.min_urgency, task.urgency);
        }
        stats.range_urgency = stats.max_urgency - stats.min_urgency;
        stats.range_importance = stats.max_importance - stats.min_importance;

        stats
    }
}
struct Tako {
    oswald: Oswald,
    current_view: View,
    target_daily_tasks: usize,
    overview_columns: usize,
    form_task: Option<Task>,
    arrange_nested_tasks: Vec<Task>,
    clear_all_dialog: bool,
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

    fn show_arrange_labels(&self, ui: &mut Ui, rect: &Rect) {
        let north_label = ui.painter().layout_no_wrap("(+) important".to_owned(), ARRANGE_LABEL_FONT, ARRANGE_FG);
        let south_label = ui.painter().layout_no_wrap("(-) important".to_owned(), ARRANGE_LABEL_FONT, ARRANGE_FG);
        let west_label = ui.painter().layout_no_wrap("(-) urgency".to_owned(), ARRANGE_LABEL_FONT, ARRANGE_FG);
        let east_label = ui.painter().layout_no_wrap("(+) urgency".to_owned(), ARRANGE_LABEL_FONT, ARRANGE_FG);

        let mut north_anchor = Align2::CENTER_TOP.pos_in_rect(&rect);
        north_anchor.x -= north_label.rect.width()/2.0;

        let mut south_anchor = Align2::CENTER_BOTTOM.pos_in_rect(&rect);
        south_anchor.x -= south_label.rect.width()/2.0;
        south_anchor.y -= south_label.rect.height();

        let mut west_anchor = Align2::LEFT_CENTER.pos_in_rect(&rect);
        west_anchor.y -= west_label.rect.height()/2.0;

        let mut east_anchor = Align2::RIGHT_CENTER.pos_in_rect(&rect);
        east_anchor.x -= east_label.rect.width();
        east_anchor.y -= east_label.rect.height()/2.0;

        ui.painter().galley(north_anchor, north_label, ARRANGE_FG);
        ui.painter().galley(south_anchor, south_label, ARRANGE_FG);
        ui.painter().galley(west_anchor, west_label, ARRANGE_FG);
        ui.painter().galley(east_anchor, east_label, ARRANGE_FG);

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
                        self.save_arrange();
                        self.arrange_nested_tasks.clear();
                    }
                    if self.tako_full_button(ui, "Arrange", matches!(self.current_view, View::Arrange)).clicked() {
                        self.current_view = View::Arrange;
                    }
                    if self.tako_full_button(ui, "Arrange (All)", matches!(self.current_view, View::ArrangeAll)).clicked() {
                        self.current_view = View::ArrangeAll;
                    }
                    if self.tako_full_button(ui, "Clear All", false).clicked() {
                        self.clear_all_dialog = true;
                    }
                });
            });
            Window::new("Are you sure?")
                .open(&mut self.clear_all_dialog)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("This will DELETE all your tasks, are you sure?");
                        if ui.button("Yes, I understand").clicked() {
                            self.oswald.clear();
                        }
                    });
                });

    }

    fn show_overview_frame(&mut self, ui: &mut Ui, ctx: &Context) {
        Frame::default()
            .show(ui, |ui| {
                let mut curr_column = 0;
                let enumerated_tasks = self.oswald.get_all_tasks().into_iter().enumerate();
                let mut pending_update_task: Option<Task> = None;
                let mut pending_deletion_id: Option<u32> = None;
                ScrollArea::vertical().show(ui, |ui| {
                    ui.columns(self.overview_columns, |columns| {
                        for (idx, task) in enumerated_tasks {
                            if idx > 0 && idx % self.target_daily_tasks == 0 && curr_column < self.overview_columns - 1 { curr_column += 1; }
                            if let Some(column) = columns.get_mut(self.overview_columns - curr_column - 1) {
                                let response = task.show_overview(column);
                                if response.hovered() {
                                    ctx.set_cursor_icon(CursorIcon::PointingHand)
                                }

                                if response.double_clicked () {
                                    if matches!(task.status, TaskStatus::Archived) {
                                        pending_deletion_id = Some(task.id);
                                    } else {
                                        let mut updated_task = task.clone();
                                        updated_task.status = match task.status {
                                            TaskStatus::Open => TaskStatus::Done,
                                            TaskStatus::Done => TaskStatus::Open,
                                            _ => TaskStatus::Archived
                                        };
                                        pending_update_task = Some(updated_task);
                                    }
                                }

                                if response.secondary_clicked () {
                                    let mut updated_task = task.clone();
                                    updated_task.status = match task.status {
                                        TaskStatus::Archived => TaskStatus::Open,
                                        _ => TaskStatus::Archived
                                    };
                                    pending_update_task = Some(updated_task);
                                }
                            }
                        }
                    });
                });
                if let Some(task) = pending_update_task {
                    self.oswald.add_task(Box::new(task));
                }
                if let Some(task_id) = pending_deletion_id {
                    self.oswald.delete_task(task_id);
                }
            });
    }

    fn show_arrange_frame(&mut self, ui: &mut Ui, ctx: &Context) {
        self.show_task_form(ctx);
        Frame::default()
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            if let Some(parent_task) = self.arrange_nested_tasks.last() {
                                ui.label(parent_task.desc.clone());

                                if ui.button("Home").clicked() {
                                    self.arrange_nested_tasks.clear();
                                };
                                if ui.button("Back").clicked() {
                                    self.arrange_nested_tasks.pop();
                                }
                            }
                        });
                        if ui.add_sized(Vec2::new(144.0, 16.0), Button::new("Add Task")).clicked() {
                            self.form_task = Some(Task::new_with_id(self.next_task_id));
                        }
                    });
                    ui.separator();
                    let (_, area_rect) = ui.allocate_space(ui.available_size());
                    Area::new("Arrange".into())
                        .movable(true)
                        .default_size(ui.available_size())
                        .constrain_to(area_rect)
                        .show(ctx, |ui| {
                            let tasks = match self.arrange_nested_tasks.last() {
                                Some(parent_task) => parent_task.get_subtasks(),
                                None => self.oswald.get_tasks()
                            };
                            let mut pending_update_task: Option<Task> = None;
                            let mut pending_deletion_id: Option<u32> = None;
                            let mut new_parent_task: Option<Task> = None;

                            for task in tasks {
                                let response = task.show_arrange(ui, &area_rect);

                                if response.hovered() {
                                    ui.ctx().set_cursor_icon(CursorIcon::Grab);
                                }

                                if response.middle_clicked() {
                                    self.form_task = Some(task.clone());
                                } 
                                if response.double_clicked() {
                                    new_parent_task = Some(task.clone());
                                }

                                if response.secondary_clicked() {
                                    if matches!(task.status, TaskStatus::Archived) {
                                        pending_deletion_id = Some(task.id);
                                    } else {
                                        let mut updated_task = task.clone();
                                        updated_task.status = TaskStatus::Archived;
                                        pending_update_task = Some(updated_task);
                                    }
                                }



                                if response.dragged() {
                                    ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                                    let delta = response.drag_delta();
                                    if delta != Vec2::ZERO {
                                        let mut task = task.clone();
                                        task.delta_update(&delta, &area_rect);
                                        pending_update_task = Some(task);
                                    }
                                }
                            }
                            self.show_arrange_labels(ui, &area_rect);

                            if let Some(task) = pending_update_task { 
                                let task = Box::new(task);
                                match &mut self.arrange_nested_tasks.last_mut() {
                                    Some(parent) => parent.add_subtask(task),
                                    None => self.oswald.add_task(task)
                                }
                                self.save_arrange();
                            }

                            if let Some(task_id) = pending_deletion_id {
                                match &mut self.arrange_nested_tasks.last_mut() {
                                    Some(parent) => parent.delete_subtask(task_id),
                                    None => self.oswald.delete_task(task_id)
                                }
                                self.save_arrange();
                            }

                            if let Some(new_parent_task) = new_parent_task.take() {
                                self.arrange_nested_tasks.push(new_parent_task);
                            }
                        });
                });
            });
    }

    fn show_arrange_all_frame(&mut self, ui: &mut Ui, ctx: &Context) {
        self.show_task_form(ctx);
        Frame::default()
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.add_sized(Vec2::new(144.0, 16.0), Button::new("Add Task")).clicked() {
                            self.form_task = Some(Task::new_with_id(self.next_task_id));
                        }
                    });
                    ui.separator();
                    let (_, area_rect) = ui.allocate_space(ui.available_size());
                    Area::new("Arrange".into())
                        .movable(true)
                        .default_size(ui.available_size())
                        .constrain_to(area_rect)
                        .show(ctx, |ui| {
                            let tasks = self.oswald.get_all_tasks();
                            let mut pending_update_task: Option<Task> = None;

                            for task in tasks {
                                let response = task.show_arrange(ui, &area_rect);

                                if response.hovered() {
                                    ui.ctx().set_cursor_icon(CursorIcon::Grab);
                                }

                                if response.triple_clicked() {
                                    self.form_task = Some(task.clone());
                                }

                                if response.dragged() {
                                    ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                                    let delta = response.drag_motion();
                                    if delta != Vec2::ZERO {
                                        let mut task = task.clone();
                                        task.delta_update(&delta, &area_rect);
                                        pending_update_task = Some(task);
                                    }
                                }
                            }
                            self.show_arrange_labels(ui, &area_rect);

                            if let Some(task) = pending_update_task {
                                self.oswald.add_task(Box::new(task));
                            }
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

                match self.arrange_nested_tasks.last_mut() {
                    Some(parent_task) => {
                        parent_task.add_subtask(task);
                    },
                    None => {
                        self.oswald.add_task(task);
                    }
                }
                self.save_arrange();
            }
        }
    }
    fn save_arrange(&mut self) {
        let mut curr: Option<Task> = None;
        for task in self.arrange_nested_tasks.iter_mut().rev() {
            if let Some(prev_task) = curr.take() {
                task.add_subtask(Box::new(prev_task));
            }
            curr = Some(task.clone());
        }
        if let Some(last_task) = curr.take() {
            self.oswald.add_task(Box::new(last_task));
        }
    }
}
impl eframe::App for Tako {
    fn save(&mut self, storage: &mut dyn Storage) {
        let tasks = self.oswald.get_tasks();
        match serde_json::to_string(&tasks) {
            Ok(tasks_str) => { 
                storage.set_string("tasks", tasks_str);
            },
            Err(err) => { println!("Couldn't save tasks: {err}") }
        }
    }
    fn auto_save_interval(&self) -> Duration { AUTO_SAVE_INTERVAL }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) { 
        self.show_menu(ctx);

        CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Overview => self.show_overview_frame(ui, ctx),
                View::Arrange => self.show_arrange_frame(ui, ctx),
                View::ArrangeAll => self.show_arrange_all_frame(ui, ctx)
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
            next_task_id,
            form_task: None,
            arrange_nested_tasks: vec![],
            current_view: View::Overview,
            target_daily_tasks: 5,
            overview_columns: 3,
            clear_all_dialog: false,
        }))
    }))
}
