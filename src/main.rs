#![cfg_attr(
    hide_console,
    windows_subsystem = "windows"
)]
use std::env;
use eframe::egui;



fn main() -> eframe::Result {
    unsafe {
        windows_sys::Win32::System::Console::AttachConsole(u32::MAX);
    }

    println!("Hello, world!");

    eframe::run_native(
        "Primes",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(PrimeApp::default())))
    )
}



struct PrimeApp {
    tab_current: usize,
    tab_names: [&'static str; 3]
}



impl Default for PrimeApp {
    fn default() -> Self {
        Self {
            tab_current: 0,
            tab_names: ["Functions", "Validation", "About"]
        }
    }
}



impl eframe::App for PrimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                for (i, name) in self.tab_names.iter().enumerate() {
                    if ui.selectable_label(self.tab_current == i, *name).clicked() {
                        self.tab_current = i;
                        println!("Switched to tab {}", name);
                    }
                }
            });

            ui.separator();
            ui.add_space(10.0);

            match self.tab_current {
                0 => {
                    ui.heading("Functions page");
                },
                1 => {
                    ui.heading("Validation page");
                },
                2 => {
                    egui::Grid::new("about_grid").show(ui, |ui| {
                            
                        ui.label("Project");
                        ui.hyperlink_to(env!("CARGO_PKG_NAME"), "https://github.com/AlparDuman/primes-rust");
                        ui.end_row();
                        
                        ui.label("Version");
                        ui.label(env!("CARGO_PKG_VERSION"));
                        ui.end_row();
                        
                        ui.label("Author");
                        ui.hyperlink_to("Alpar Duman", "https://alparduman.de/");
                        ui.end_row();
                        
                        ui.label("License");
                        ui.hyperlink_to("GNU General Public License v3.0", "https://github.com/AlparDuman/primes-rust/blob/main/LICENSE");
                        ui.end_row();

                    });
                },
                _ => {}
            }

        });
    }
}


/*fn pause() {
    let mut stdout = io::stdout();
    print!("Press Enter to continue..");
    stdout.flush().unwrap();
    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer).unwrap();
}*/
