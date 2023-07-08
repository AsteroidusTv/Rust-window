use gtk4::glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(0..100);
    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(move |app| on_activate(app, number));

    // Run the application
    app.run();
}

// When the application is launched…
fn on_activate(application: &gtk4::Application, number: i32) {
    // … create a new window …
    let window = gtk4::ApplicationWindow::new(application);

    // Create a vertical box layout to hold the widgets
    let box_layout = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    // Create a text input entry
    let text_entry = gtk4::Entry::new();
    text_entry.set_placeholder_text(Some("Enter your number"));

    // Create a button
    let button = gtk4::Button::with_label("Print Text");

    // Create a label
    let label = gtk4::Label::new(None);

    // Handle button click event
    button.connect_clicked(clone!(@weak text_entry, @weak label => move |_| {
        let text = text_entry.text();
        println!("Entered text: {:?}", text);
        let enter = text.parse::<i32>().unwrap();
        let response = game(enter, number);
        label.set_text(response);
    }));

    // Set margin to box 
    box_layout.set_margin_start(10);
    box_layout.set_margin_end(10);
    box_layout.set_margin_top(10);
    box_layout.set_margin_bottom(10);

    // Add the widgets to the box layout
    box_layout.append(&text_entry);
    box_layout.append(&button);
    box_layout.append(&label);

    // Add the box layout to the window
    window.set_child(Some(&box_layout));

    window.present();
}

fn game(entered_number: i32, target_number: i32) -> &'static str {

    if entered_number > target_number {
        "Plus petit"
    } else if entered_number < target_number {
        "Plus grand"
    } else {
        "Gagné"
    }
}
