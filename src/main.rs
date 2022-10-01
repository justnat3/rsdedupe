use eframe::{
    egui::Align,
    egui::Button,
    egui::CentralPanel,
    egui::Layout,
    egui::TopBottomPanel,
    run_native, App, NativeOptions,
};
use rfd::FileDialog;
use std::path::PathBuf;

static APPNAME: &'static str = "RSDEDUPE";

enum DirChoice {
    src,
    bckup,
}

#[derive(Default)]
struct RsDeDupe {
    src_dir: Option<PathBuf>,
    bckup_dir: Option<PathBuf>,
}

impl RsDeDupe {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn get_dir(&mut self, choice: DirChoice) {
        match choice {
            DirChoice::src => {
                self.src_dir = FileDialog::new().pick_folder();
            }
            DirChoice::bckup => {
                self.bckup_dir = FileDialog::new().pick_folder();
            }
        }
    }

    fn alloc_header(&self, ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        TopBottomPanel::top("options_panel").show(ctx, |ui| {
            ui.add_space(5.);
            ui.heading("RSDEDUPE");
            ui.add_space(11.);
            eframe::egui::menu::bar(ui, |ui| {
                ui.with_layout(
                    Layout::top_down(Align::Min).with_cross_align(Align::Min),
                    |ui| {
                        ui.add_space(2.);
                        if ui.add(Button::new("🎯 Target Directory")).clicked() {
                            self.get_dir(DirChoice::src);
                        };

                        if ui
                            .add(Button::new("❌ A place to put the dupelicates"))
                            .clicked()
                        {
                            self.get_dir(DirChoice::bckup);
                        };
                    },
                );
            });
        });
    }
}

impl App for RsDeDupe {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {});
    }
}

fn main() {
    let win_option = NativeOptions::default();
    run_native(
        APPNAME,
        win_option,
        Box::new(|a| Box::new(RsDeDupe::new(a))),
    );
}
