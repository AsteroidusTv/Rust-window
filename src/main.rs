use rand::{distributions::Alphanumeric, Rng};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation};


fn main() {
    let app = Application::builder()
        .application_id("Password generator")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("Get password")
        .margin_top(100)
        .margin_start(12)
        .margin_end(12)
        .build();



    let button = Button::builder()
        .label("GET RANDOM PASSWORD")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("Password generator")
        .application(app)
        .child(&content)
        .build();

    window.set_default_size(780, 480);
    button.set_size_request(120, 60);

    button.connect_clicked(move |_| {
        let result = get_password();
        label.set_text(&result);
    });

    window.show();
}

fn get_password() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(get_random_number())
        .map(char::from)
        .collect();
    s
}

fn get_random_number() -> usize {

    let mut rng = rand::thread_rng();
    rng.gen_range(14..20)

}
