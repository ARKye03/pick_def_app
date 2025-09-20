// Object holding the state
use crate::desktop_entries::DesktopEntryManager;
use crate::mimetype_manager::MimetypeManager;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, glib};
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/window/window.blp")]
pub struct Window {
    pub desktop_manager: RefCell<DesktopEntryManager>,
    pub mimetype_manager: RefCell<Option<MimetypeManager>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Initialize managers
        let mut desktop_manager = DesktopEntryManager::new();
        if let Err(e) = desktop_manager.load_entries() {
            eprintln!("Failed to load desktop entries: {}", e);
        }
        self.desktop_manager.replace(desktop_manager);

        match MimetypeManager::new() {
            Ok(mimetype_manager) => {
                self.mimetype_manager.replace(Some(mimetype_manager));
            }
            Err(e) => {
                eprintln!("Failed to initialize mimetype manager: {}", e);
            }
        }
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all adwaita application windows
impl adw::subclass::application_window::AdwApplicationWindowImpl for Window {}
