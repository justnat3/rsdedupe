use eframe::{
    egui::Align, egui::Button, egui::CentralPanel, egui::Context, egui::Layout,
    egui::TopBottomPanel, run_native, App, NativeOptions,
};
use rfd::FileDialog;
use std::{collections::HashMap, fs::read, path::{PathBuf, Path}, ops::Index};

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
            ui.add_space(11.);
            eframe::egui::menu::bar(ui, |ui| {
                ui.with_layout(
                    Layout::top_down(Align::Min).with_cross_align(Align::Min),
                    |ui| {
                        ui.add_space(2.);
                        if ui
                            .add_sized([120., 40.], Button::new("🎯 Target Directory"))
                            .clicked()
                        {
                            self.get_dir(DirChoice::Src);
                        };

                        if ui
                            .add_sized(
                                [120., 40.],
                                Button::new("❌ A place to put the dupelicates"),
                            )
                            .clicked()
                        {
                            self.get_dir(DirChoice::Backup);
                        };
                    },
                );
            });
            ui.add_space(2.);
            ui.add_space(11.);
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
        self.render_header(ctx);
        self.render_footer(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Indexing");
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

// FileMap Stuff
//-------------------------------------------------------------------------------------

struct FileMap<'a> {
    map: HashMap<String, &'a Path>,
    matches: i64,
}

enum FileState {
    Dupelicate,
    Inserted,
    Unknown
}

// struct IndexedFile {
//     /// A File Digest
//     hash: String,
//     /// Where the digest came from
//     path: PathBuf,
// }

impl FileMap<'_> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            matches: 0,
        }
    }

    // fn index_files(&self) {
    //     for entry in &self.file_vector {
    //         let hash = self.hash_at_path(entry.to_path_buf());

    //         if let Some(hash) = hash {
    //             self.maybe_insert_hash(hash.as_str());
    //         }
    //     }
    // }

    // digest a file at some path, then return the hash and path into a struct 
    // I suppose we could fix the lifetime issue by moving the struct def outside of this
    // function, but again... I am lazy- we are programmers right?
    fn hash_at_path(&self, path: PathBuf) -> Result<String, std::io::Error> {
        // we clone here because of a lifetime issue with IndexedFile
        // I suppose we could fix that, however I am lazy
        let _xpath = path.clone();
        let bytes = read(_xpath);

        match bytes {
            Ok(v) => {
                // TODO: Remove this unwrap()
                let file_bytes = Some(v).unwrap();

                // digest file bytes (32b each) and store in another struct for later
                Ok(sha256::digest_bytes(&file_bytes))
            }
            // "bubble up, the io error"
            Err(e) => Err(e)
        }
    }

    // Here we figure out whether the hash already exists, if so we can later on move
    // the file over to the dupelicates directory location specified by the user
    fn is_duplicate(&mut self, hash: &str, file: &Path) -> FileState {
        match self.map.contains_key(hash) {
            true => FileState::Dupelicate,
            false => {
                self.map.insert(hash.to_string(), &file);
                FileState::Inserted
            }
        }
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

    let mut map = FileMap::new();

    for entry in paths {
        let hash = map.hash_at_path(entry);
        match hash {
            Ok(v) => {
                map.is_duplicate(&v, &entry);
            },
            Err(_) => todo!()
        }
    }



    // pop file from stack
}
