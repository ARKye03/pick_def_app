mod window;

use adw::Application;
use freedesktop_desktop_entry::*;
use gtk::glib;
use gtk::prelude::*;
use window::Window;

const APP_ID: &str = "com.github.arkye03.app_defaulter";

fn main() -> glib::ExitCode {
    let locales = get_languages_from_env();

    let entries = Iter::new(default_paths())
        .entries(Some(&locales))
        .collect::<Vec<_>>();

    for entry in entries {
        let path_src = PathSource::guess_from(&entry.path);

        println!("{:?}: {}\n---\n{}", path_src, entry.path.display(), entry);
    }

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
