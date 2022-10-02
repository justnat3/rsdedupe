use eframe::{
    egui::Align, egui::Button, egui::CentralPanel, egui::Context, egui::Layout,
    egui::TopBottomPanel, run_native, App, NativeOptions,
};
use rfd::FileDialog;
use std::{
    collections::HashMap,
    fs::{read},
    path::PathBuf,
};


static APPNAME: &'static str = "RSDEDUPE";

enum DirChoice {
    Src,
    Backup,
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
            DirChoice::Src => {
                self.src_dir = FileDialog::new().pick_folder();
            }
            DirChoice::Backup => {
                self.bckup_dir = FileDialog::new().pick_folder();
            }
        }
    }

    fn render_header(&mut self, ctx: &Context) {
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
                            self.get_dir(DirChoice::Src);
                        };

                        if ui
                            .add(Button::new("❌ A place to put the dupelicates"))
                            .clicked()
                        {
                            self.get_dir(DirChoice::Backup);
                        };
                    },
                );
            });
        });
    }

    // render footer will just contain the "submit button for the lack of anything else"
    fn render_footer(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("submit_panel").show(ctx, |ui| {
            ui.add_space(5.);
            ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
                ui.add_space(5.);
                ui.add(Button::new("Run Indexing Job"));
            });
        });
    }
}

impl App for RsDeDupe {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |_| {
            self.render_header(ctx);
            self.render_footer(ctx);
        });
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

struct FileMap {
    map: HashMap<String, PathBuf>,
    matches: i64,
    file_vector: Vec<PathBuf>,
}

impl FileMap {
    fn new(files: Vec<PathBuf>) -> Self {
        Self {
            map: HashMap::new(),
            matches: 0,
            file_vector: files,
        }
    }

    fn index_files(&self) {
        for i in &self.file_vector {
            let buf = i.to_path_buf();
            let hash = self.hash_at_path(buf);
            if let Some(hash) = hash {
                self.maybe_insert_hash(hash.as_str());
            }
        }
    }

    // grab hash of a path
    fn hash_at_path(&self, path: PathBuf) -> Option<String> {
        let bytes = read(path);
        match bytes {
            Ok(v) => Some(sha256::digest_bytes(&v)),
            Err(_) => None
        }
    }

    // insert hash to the file map
    fn maybe_insert_hash(&self, hash: &str) {
        if self.map.contains_key(hash) {

        }
    }
    // check for hash collisions
    fn differs(&self) {
        todo!()
    }
}

fn dummy_func() {
    // initialize map with some files to roll through
    let mut paths: Vec<PathBuf> = Vec::new();
    if let Ok(i) = std::fs::read_dir("./dummy_dir") {

        for entry in i {
            match entry {
                Ok(v) => paths.push(v.path()),
                Err(_) => todo!(),
            }
        }
    };

    let map = FileMap::new(paths);

    map.index_files();

    // pop file from stack
}
