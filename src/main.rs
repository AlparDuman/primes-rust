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
    // println!("{}", i64::MAX); // 9223372036854775807
    // println!("{}", u64::MAX); // 18446744073709551615

    eframe::run_native(
        "Primes Rust",
        eframe::NativeOptions {
            viewport:egui::ViewportBuilder::default()
                .with_title("Primes Rust")
                .with_inner_size([286.0, 286.0])
                .with_min_inner_size([286.0, 286.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(PrimeApp::default())))
    )
}



struct PrimeApp {
    tab_current: usize,
    tab_names: [&'static str; 4],
    input_start_value: String,
    input_start_value_previous: String,
    input_range_value: String,
    input_range_value_previous: String,
    output_results_value: String
}



impl Default for PrimeApp {
    fn default() -> Self {
        Self {
            tab_current: 0,
            tab_names: ["Functions", "Validation", "Explanation", "About"],
            input_start_value: String::new(),
            input_start_value_previous: String::new(),
            input_range_value: String::new(),
            input_range_value_previous: String::new(),
            output_results_value: String::new(),
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
            ui.add_space(5.0);

            match self.tab_current {
                0 => {

                    egui::Grid::new("about_grid")
                        .min_col_width(0.0)
                        .show(ui, |ui| {

                        ui.label("Start");

                        let input_start = ui.add_sized(
                            [141.0, 20.0],
                            egui::TextEdit::singleline(&mut self.input_start_value)
                                .font(egui::TextStyle::Monospace)
                        );
                        if input_start.changed() {
                            self.input_start_value.retain(|c| c.is_ascii_digit());

                            if self.input_start_value.is_empty() {
                                self.input_start_value_previous.clear();
                            } else {
                                match self.input_start_value.parse::<i64>() {
                                    Ok(_) => {
                                        self.input_start_value = self.input_start_value.trim_start_matches('0').to_string();
                                        self.input_start_value_previous = self.input_start_value.clone();
                                    }
                                    Err(_) => {
                                        self.input_start_value = self.input_start_value_previous.clone();
                                    }
                                }
                            }
                        }

                        if ui.button("Check").clicked() {
                            println!("Pressed button 'Check'");
                        }

                        ui.label("");
                        
                        if ui.ctx().screen_rect().width() < 528.0 {
                            ui.end_row();
                        }

                        ui.label("Range");

                        let input_range = ui.add_sized(
                            [141.0, 20.0],
                            egui::TextEdit::singleline(&mut self.input_range_value)
                                .font(egui::TextStyle::Monospace)
                        );
                        if input_range.changed() {
                            self.input_range_value.retain(|c| c.is_ascii_digit());

                            if self.input_range_value.is_empty() {
                                self.input_range_value_previous.clear();
                            } else {
                                match self.input_range_value.parse::<i64>() {
                                    Ok(_) => {
                                        self.input_range_value_previous = self.input_range_value.clone();
                                    }
                                    Err(_) => {
                                        self.input_range_value = self.input_range_value_previous.clone();
                                    }
                                }
                            }
                        }

                        if ui.button("Count").clicked() {
                            println!("Pressed button 'Count'");
                        }

                        if ui.button("List").clicked() {
                            println!("Pressed button 'List'");
                        }

                        ui.end_row();

                    });

                    ui.add_space(5.0);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(ui.available_height() - 25.0)
                        .show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(&mut self.output_results_value)
                                    .font(egui::TextStyle::Monospace)
                                    .lock_focus(true)
                                    .interactive(false)
                            );
                        });
                    
                    ui.add_space(5.0);
                    let output_save = ui.add_sized(
                        [ui.available_width(), 0.0],
                        egui::Button::new("Save")
                    );
                    if output_save.clicked() {
                        println!("Pressed button 'Save'");
                    }
                },
                1 => {
                    ui.heading("Validation page");
                },
                2 => {
                    ui.heading("Explanation page");
                },
                3 => {
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
