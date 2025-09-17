mod window;

use gtk::prelude::*;
use gtk::{Application, glib};
use window::Window;

const APP_ID: &str = "org.gtk_rs.CompositeTemplates1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
