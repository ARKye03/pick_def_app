// Object holding the state
use crate::desktop_entries::DesktopEntryManager;
use crate::mimetype_manager::MimetypeManager;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, ToggleButton, glib};
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/window/window.blp")]
pub struct Window {
    #[template_child]
    pub filter_wrap_box: TemplateChild<adw::WrapBox>,
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

        // Populate filter buttons with categories
        self.populate_filter_buttons();
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

impl Window {
    pub fn populate_filter_buttons(&self) {
        let desktop_manager = self.desktop_manager.borrow();
        let main_types = desktop_manager.get_main_mimetype_categories();

        // Clear existing children
        while let Some(child) = self.filter_wrap_box.first_child() {
            self.filter_wrap_box.remove(&child);
        }

        // Add toggle buttons for each main mimetype category
        for main_type in main_types {
            let button = ToggleButton::builder().label(&main_type).build();

            // Add some styling
            button.add_css_class("pill");

            self.filter_wrap_box.append(&button);
        }
    }
}
