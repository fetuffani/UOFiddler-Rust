//! egui side of the auto-updater (the logic is in `updater.rs`): background check, the "new
//! version" dialog, download progress and the options controls.

use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;

use eframe::egui;
use uo_core::FileManager;

use crate::updater::{self, PackageKind, Progress, Release, UpdateError};

enum Msg {
    Checked(Result<Release, UpdateError>),
    Updated(Result<(), UpdateError>),
}

enum Stage {
    Idle,
    Checking,
    UpToDate,
    Available(Release),
    Working(Release),
    Failed(UpdateError),
}

pub struct UpdateUi {
    stage: Stage,
    started: bool,
    manual: bool,
    rx: Option<Receiver<Msg>>,
    progress: Arc<Progress>,
    install: updater::Install,
}

impl UpdateUi {
    pub fn new() -> Self {
        Self {
            stage: Stage::Idle,
            started: false,
            manual: false,
            rx: None,
            progress: Arc::new(Progress::default()),
            install: updater::detect_current_install(),
        }
    }

    /// Starts a check in a background thread. `manual` = the user asked: errors and "up to date"
    /// are shown, and an ignored version is offered again. Automatic checks stay silent.
    pub fn check(&mut self, ctx: &egui::Context, manual: bool) {
        if matches!(self.stage, Stage::Checking | Stage::Working(_)) {
            return;
        }
        self.manual = manual;
        self.stage = if manual { Stage::Checking } else { Stage::Idle };
        let (tx, rx) = channel();
        self.rx = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(Msg::Checked(updater::fetch_latest()));
            ctx.request_repaint();
        });
    }

    fn start_update(&mut self, ctx: &egui::Context, release: Release) {
        self.progress = Arc::new(Progress::default());
        let (tx, rx) = channel();
        self.rx = Some(rx);
        let (p, install, rel, ctx2) = (self.progress.clone(), self.install.clone(), release.clone(), ctx.clone());
        std::thread::spawn(move || {
            let _ = tx.send(Msg::Updated(updater::run_update(&rel, &install, &p, false)));
            ctx2.request_repaint();
        });
        self.stage = Stage::Working(release);
    }

    fn error_text(e: &UpdateError) -> String {
        t!(e.key(), detail = e.detail())
    }

    /// Options-dialog controls: automatic check toggle, "Check for updates" button, version.
    pub fn settings_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, fm: &mut FileManager) {
        if ui.checkbox(&mut fm.auto_update_check, t!("update.auto")).changed() {
            fm.save_settings();
        }
        ui.horizontal(|ui| {
            let busy = matches!(self.stage, Stage::Checking | Stage::Working(_));
            if ui.add_enabled(!busy, egui::Button::new(t!("update.check"))).clicked() {
                self.check(ctx, true);
            }
            ui.label(t!("update.version", version = Self::version_label()));
        });
    }

    fn version_label() -> String {
        if updater::current_build() == 0 {
            t!("update.dev", version = updater::current_version())
        } else {
            updater::current_version()
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, fm: &mut FileManager) {
        if !self.started {
            self.started = true;
            updater::cleanup_backup();
            updater::confirm_started();
            if fm.auto_update_check {
                self.check(ctx, false);
            }
        }
        if let Some(msg) = self.rx.as_ref().and_then(|rx| rx.try_recv().ok()) {
            match msg {
                Msg::Checked(Ok(r)) => {
                    let ignored = fm.ignored_update_build == Some(r.build);
                    self.stage = if updater::is_newer(updater::current_build(), r.build) {
                        if self.manual || !ignored { Stage::Available(r) } else { Stage::Idle }
                    } else if self.manual {
                        Stage::UpToDate
                    } else {
                        Stage::Idle
                    };
                }
                Msg::Checked(Err(e)) => self.stage = if self.manual { Stage::Failed(e) } else { Stage::Idle },
                Msg::Updated(Ok(())) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                Msg::Updated(Err(e)) => self.stage = Stage::Failed(e),
            }
        }

        match std::mem::replace(&mut self.stage, Stage::Idle) {
            Stage::Idle => {}
            Stage::Checking => {
                self.stage = Stage::Checking;
                egui::Window::new(t!("update.title")).collapsible(false).resizable(false).show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(t!("update.checking"));
                    });
                });
            }
            Stage::UpToDate => {
                let mut open = true;
                egui::Window::new(t!("update.title")).collapsible(false).resizable(false).show(ctx, |ui| {
                    ui.label(t!("update.uptodate", version = Self::version_label()));
                    if ui.button(t!("common.close")).clicked() {
                        open = false;
                    }
                });
                if open {
                    self.stage = Stage::UpToDate;
                }
            }
            Stage::Failed(e) => {
                let mut open = true;
                egui::Window::new(t!("update.failed")).collapsible(false).resizable(false).show(ctx, |ui| {
                    ui.label(Self::error_text(&e));
                    if ui.button(t!("common.close")).clicked() {
                        open = false;
                    }
                });
                if open {
                    self.stage = Stage::Failed(e);
                }
            }
            Stage::Available(r) => {
                let page_only = self.install.kind == PackageKind::PageOnly
                    || updater::select_asset(&r.assets, self.install.kind).is_none();
                let mut next = None;
                egui::Window::new(t!("update.title")).collapsible(false).resizable(true).default_width(460.0).show(ctx, |ui| {
                    ui.label(t!("update.available", version = r.version, current = Self::version_label()));
                    if !r.notes.trim().is_empty() {
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(t!("update.notes")).strong());
                        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                            let notes: String = r.notes.chars().take(3000).collect();
                            ui.label(notes);
                        });
                    }
                    ui.hyperlink_to(t!("update.release_page"), &r.page_url);
                    if page_only {
                        ui.add_space(6.0);
                        ui.label(t!("update.pageonly"));
                    }
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if page_only {
                            if ui.button(t!("update.open_page")).clicked() {
                                ctx.open_url(egui::OpenUrl::new_tab(&r.page_url));
                                next = Some(Stage::Idle);
                            }
                        } else if ui.button(t!("update.now")).clicked() {
                            next = Some(Stage::Working(r.clone()));
                        }
                        if ui.button(t!("update.later")).clicked() {
                            next = Some(Stage::Idle);
                        }
                        if ui.button(t!("update.ignore")).clicked() {
                            fm.ignored_update_build = Some(r.build);
                            fm.save_settings();
                            next = Some(Stage::Idle);
                        }
                    });
                });
                match next {
                    Some(Stage::Working(rel)) => self.start_update(ctx, rel),
                    Some(s) => self.stage = s,
                    None => self.stage = Stage::Available(r),
                }
            }
            Stage::Working(r) => {
                let phase = self.progress.phase.load(Ordering::Relaxed);
                let done = self.progress.done.load(Ordering::Relaxed);
                let total = self.progress.total.load(Ordering::Relaxed);
                egui::Window::new(t!("update.title")).collapsible(false).resizable(false).show(ctx, |ui| {
                    ui.set_min_width(360.0);
                    match phase {
                        0 => {
                            let frac = if total > 0 { done as f32 / total as f32 } else { 0.0 };
                            ui.label(t!(
                                "update.downloading",
                                done = format!("{:.1}", done as f64 / 1_048_576.0),
                                total = format!("{:.1}", total as f64 / 1_048_576.0)
                            ));
                            ui.add(egui::ProgressBar::new(frac).show_percentage());
                            if ui.button(t!("update.cancel")).clicked() {
                                self.progress.cancel.store(true, Ordering::Relaxed);
                            }
                        }
                        1 => {
                            ui.label(t!("update.verifying"));
                        }
                        2 => {
                            ui.label(t!("update.installing"));
                        }
                        _ => {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label(t!("update.restarting"));
                            });
                        }
                    }
                });
                self.stage = Stage::Working(r);
                ctx.request_repaint_after(std::time::Duration::from_millis(100));
            }
        }
    }
}
