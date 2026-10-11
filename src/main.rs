#![cfg_attr(
    hide_console,
    windows_subsystem = "windows"
)]
use std::env;
use eframe::egui;



    }






// Implementation Primes

pub struct Primes {
    /*cache_small: Vec<usize>,
    cache_small_size: usize,
    demask: [(u8, usize); 8]*/
}



impl Primes {
    pub fn new() -> Self {
        /*let cache_small = Vec::new();
        let cache_small_size = 0;
        let demask = [(0, 0); 8];*/

        Primes {
            /*cache_small,
            cache_small_size,
            demask*/
        }
    }

    pub fn test(&self, buffer: &mut String) {
        fn is_test(number: isize) -> bool {
            if number < 2 || (number != 2 && number % 2 == 0) {
                return false;
            }
            let limit = number.isqrt();
            for divisor in (3..=limit).step_by(2) {
                if number % divisor == 0 {
                    return false;
                }
            }
            true
        }

        fn list_test<F>(range: isize, mut start: isize, mut action: F) where F: FnMut(isize) {
            let end = start + range;
            if range < 1 || end < start {
                return;
            }
            if start <= 2 && end >= 2 {
                action(2);
            }
            if start % 2 == 0 {
                start += 1;
            }
            for number in (start..=end).step_by(2) {
                if is_test(number) {
                    action(number);
                }
            }
        }

        fn count_test(range: isize, start: isize) -> isize {
            let mut primes = 0;
            list_test(range, start, |_| primes += 1);
            primes
        }

        fn get_test(range: isize, start: isize) -> Vec<isize> {
            let mut primes = Vec::new();
            list_test(range, start, |number| primes.push(number));
            primes
        }

        let mut measure_begin;
        let mut measure_duration;
        let start = 0;
        let range = 20;
        buffer.clear();

        measure_begin = Instant::now();
        let measure_result = count_test(range, start);
        measure_duration = measure_begin.elapsed().as_secs_f64() * 1000.0;
        buffer.push_str(&format!("Reference counted {} primes in {:.2}ms", measure_result.to_string(), measure_duration));

        measure_begin = Instant::now();
        let measure_result = get_test(range, start);
        measure_duration = measure_begin.elapsed().as_secs_f64() * 1000.0;
//        let measure_result_len = measure_result.len();
//        if measure_result_len > 6 {
            let measure_result = format!("{:?},..,{:?}", &measure_result[0..3], &measure_result[(measure_result.len() - 3)..]);
//        }
/*        if measure_result.len() < 6 {
            let measure_result = measure_result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        } else {
            let measure_result = format!("{:?},..,{:?}", &measure_result[0..3], &measure_result[(measure_result.len() - 3)..]);
        }*/
        buffer.push_str(&format!("\nReference listed {:?} primes in {:.2}ms", measure_result, measure_duration));













//        println!("{}",  is_test(1000099));
//        println!("{:?}", get_test(100, 1000000));
//        println!("{:?}", count_test(100, 1000000));
    }
}






// Implementation PrimeApp

struct PrimeApp {
    engine: Primes,
    tab_current: usize,
    tab_names: [&'static str; 4],
    input_start_value: String,
    input_start_value_previous: String,
    input_range_value: String,
    input_range_value_previous: String,
    output_results_value: String,
    buffer_test: String
}

impl Default for PrimeApp {
    fn default() -> Self {
        Self {
            engine: Primes::new(),
            tab_current: 0,
            tab_names: ["Functions", "Validation", "Explanation", "About"],
            input_start_value: String::new(),
            input_start_value_previous: String::new(),
            input_range_value: String::new(),
            input_range_value_previous: String::new(),
            output_results_value: String::new(),
            buffer_test: String::new()
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
                                match self.input_start_value.parse::<isize>() {
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
                                match self.input_range_value.parse::<isize>() {
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
                    let run_self_test = ui.add_sized(
                        [ui.available_width(), 0.0],
                        egui::Button::new("Run Self-Test")
                    );
                    if run_self_test.clicked() {
                        println!("Pressed button 'Run Self-Test'");
                        self.engine.test(&mut self.buffer_test);
                    }
                    ui.add_space(5.0);
                    ui.label(&self.buffer_test);
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






// Main

fn main() -> eframe::Result {
    unsafe {
        windows_sys::Win32::System::Console::AttachConsole(u32::MAX);
    }

    println!("Hello, world!");
    // println!("{}", i64::MAX); // 9223372036854775807
    // println!("{}", u64::MAX); // 18446744073709551615
//    let engine = Primes::new();
//    engine.test();

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


/*fn pause() {
    let mut stdout = io::stdout();
    print!("Press Enter to continue..");
    stdout.flush().unwrap();
    let mut _buffer = String::new();
    io::stdin().read_line(&mut _buffer).unwrap();
}*/
