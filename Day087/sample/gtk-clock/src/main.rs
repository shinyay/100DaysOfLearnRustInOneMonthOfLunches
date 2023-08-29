extern crate gtk;
extern crate chrono;

use gtk::prelude::*;
use gtk::{Label, Window, WindowType};
use chrono::prelude::*;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Japan Clock");
    window.set_default_size(300, 150);

    // Create a label to display the time
    let time_label = Label::new(None);
    time_label.set_font(&"Helvetica 48");
    window.add(&time_label);

    // Update the time label
    update_time(&time_label);

    // Set up a timer to update the time every second
    gtk::timeout_add_seconds(1, clone!(@weak time_label => move || {
        update_time(&time_label);
        Continue(true)
    }));

    // Connect the window close event
    window.connect_delete_event(|_, _| {
        // Terminate the GTK main loop
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all widgets
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}

fn update_time(label: &Label) {
    let japan_time = Utc::now() + chrono::Duration::hours(9); // Japan is UTC+9
    let formatted_time = japan_time.format("%H:%M:%S").to_string();
    label.set_text(&formatted_time);
}
