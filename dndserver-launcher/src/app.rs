use std::{env, process::{Child, Command}, time::Instant, path::Path};

use crate::{memory};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppContext {
    dnd_binary_path: String,
    dnd_binary_arguments: String,
    dll_path: String,
    #[serde(skip)]
    process: Option<SpawnedProcess>,
    timeout: u64
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            dnd_binary_path: String::default(),
            dnd_binary_arguments: "-server=dcweb.pages.dev:80".to_owned(),
            dll_path: App::get_patch_location(),
            process: None,
            timeout: 15
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    data: Box<AppContext>
}

impl Default for App {
    fn default() -> Self {
        let data = AppContext::default();        
        Self {
            data: Box::new(data)
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts);    
        cc.egui_ctx.set_fonts(fonts);
        
        if let Some(storage) = cc.storage {
            if let Some(t) = eframe::get_value::<App>(storage, eframe::APP_KEY) {                
                return t;
            }   
        }

        Default::default()
    }

    #[cfg(debug_assertions)]
    fn get_patch_location() -> String {
        let dir = env::current_dir().unwrap();
        format!(r"{}/target/debug/dndserver_patch.dll", dir.display().to_string())
    }

    #[cfg(not(debug_assertions))]
    fn get_patch_location() -> String {
        let dir = env::current_dir().unwrap();
        format!(r"{}/dndserver_patch.dll", dir.display().to_string())
    }

    fn dll_valid() -> bool {
        let path = App::get_patch_location();
        Path::new(&path).exists()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;

        let is_dll_valid = App::dll_valid();

        match data.process.as_mut() {
            Some(process) => {
                if process.is_ready() { 
                    let pid = process.handle.id();
                    if process.is_doa {
                        process.handle.kill().unwrap();
                        let handle = Command::new(data.dnd_binary_path.clone())
                        .arg(data.dnd_binary_arguments.to_owned())
                        .spawn().unwrap();
                    
                        data.process = Some(SpawnedProcess::new(handle, false));
                    }
                    else {
                        if is_dll_valid {
                            memory::inject(pid, data.dll_path.as_str());    
                        }
                        data.process = None;
                    }
                }
                ctx.request_repaint();
            },
            None => {}
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(r"Select the DungeonCrawler.exe binary in 'Dark and Darker A5\DungeonCrawler\Binaries\Win64', not DungeonCrawler.exe in the root folder.");
            ui.allocate_space(egui::vec2(1f32, 5f32));
            if ui.button("Select DungeonCrawler.exe location...").clicked() {
                match rfd::FileDialog::new()
                    .add_filter("exe", &["exe"])
                    .pick_file() {
                        Some(path) => {
                            data.dnd_binary_path = path.display().to_string();
                        },
                        _ => ()
                }
            };
            ui.add(egui::Label::new(format!("{}", data.dnd_binary_path)).wrap(true));

            ui.separator();

            ui.allocate_space(egui::vec2(1f32, 2f32));
            ui.label("Launch arguments:");
            ui.add(egui::TextEdit::singleline(&mut data.dnd_binary_arguments).hint_text("Launch args"));

            ui.allocate_space(egui::vec2(1f32, ui.available_height() - 60f32));
            ui.vertical_centered(|ui| {
                let text = match is_dll_valid {
                    true => "found patch dll",
                    false => "dndserver_patch.dll not found - will launch without injecting"
                };
                ui.label(text);
            });

            ui.allocate_space(egui::vec2(1f32, ui.available_height() - 40f32));
            if data.process.is_none() {
                ui.vertical_centered(|ui| {
                    if ui.button("Launch").clicked() {
                        let p = memory::get_process_list("dungeoncrawler.exe");

                        let handle = Command::new(data.dnd_binary_path.clone())
                            .arg(data.dnd_binary_arguments.to_owned())
                            .spawn()
                            .unwrap();

                        let is_doa = match p {
                            Ok(p) => !p.is_empty(),
                            Err(_) => false
                        };

                        data.process = Some(SpawnedProcess::new(handle, is_doa));
                    }
                });
            }
        });

        egui::TopBottomPanel::bottom("footer").max_height(22f32).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.allocate_space(egui::vec2(ui.available_width() - 57f32, 1f32));
                ui.hyperlink_to(format!("{} github", egui_phosphor::GITHUB_LOGO), "https://github.com/mfloob/dndserver-launcher");
            });
        });
    }
}

struct SpawnedProcess {
    handle: Child,
    time_spawned: Instant,
    is_doa: bool,
}

impl SpawnedProcess {
    fn new(handle: Child, is_doa: bool) -> Self {
        Self {
            handle,
            time_spawned: Instant::now(),
            is_doa,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.time_spawned.elapsed().as_millis() >= 400        
    }
}