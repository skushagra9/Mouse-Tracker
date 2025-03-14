use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::utils;

pub struct MyApp {
    task: String,
    running: Arc<Mutex<bool>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { task: String::new(), running: Arc::new(Mutex::new(false)) }
    }
}

impl MyApp {
    fn start_task(&self) {
        let running_clone = Arc::clone(&self.running);
        let task_clone = self.task.clone();
        thread::spawn(move || {
            {
                let mut running = running_clone.lock().expect("Failed to lock mutex");
                *running = true; 
            }
            utils::get_mouse_movements(&running_clone, task_clone);

        });
    }

    fn stop_task(&self) {
        let mut running = self.running.lock().expect("Failed to lock mutex");
        *running = false;
    }

    fn get_status(&self) -> &str {
        let running = self.running.lock().expect("Failed to lock mutex");
        if *running { "Status: Running" } else { "Status: Stopped" }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Movements Tracking App");

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Enter your task name:");
                ui.text_edit_singleline(&mut self.task);
            });
            ui.horizontal(|ui| {
                if ui.button("Start").clicked() {
                    self.start_task();
                    println!("Clicked {}", self.task);
                }
                ui.add_space(5.0);
                if ui.button("Stop").clicked() {
                    self.stop_task();
                    println!("Stopped");
                }
            });
            ui.add_space(10.0);
            ui.label(self.get_status());
        });
    }
}