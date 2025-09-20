mod desktop_entries;
mod mimetype_manager;
mod window;

use adw::Application;
use gtk::glib;
use gtk::prelude::*;
use window::Window;

const APP_ID: &str = "com.github.arkye03.app_defaulter";

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
