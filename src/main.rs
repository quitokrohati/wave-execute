use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, TextView, Box};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.quitokrohati.waveexecutor"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Wave Executor");
        window.set_default_size(400, 300);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let text_view = TextView::new();
        let execute_button = Button::with_label("Execute Script");

        execute_button.connect_clicked(move |_| {
            let script = text_view.get_buffer().unwrap().get_text(&text_view.get_buffer().unwrap().get_start_iter(), &text_view.get_buffer().unwrap().get_end_iter(), false).unwrap();
            execute_script(script);
        });

        vbox.pack_start(&text_view, true, true, 0);
        vbox.pack_start(&execute_button, false, false, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}

fn execute_script(script: String) {
    let output = Command::new("cmd")
        .arg("/C")
        .arg(format!("start roblox-player.exe -script {}", script))
        .output()
        .expect("Failed to execute script");
    println!("Output: {:?}", output);
}