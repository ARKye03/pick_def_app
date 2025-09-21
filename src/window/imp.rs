// Object holding the state
use crate::desktop_entries::DesktopEntryManager;
use crate::mimetype_manager::MimetypeManager;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Entry, Label, ToggleButton, glib};
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/window/window.blp")]
pub struct Window {
    #[template_child]
    pub filter_entry: TemplateChild<Entry>,
    #[template_child]
    pub filter_wrap_box: TemplateChild<adw::WrapBox>,
    #[template_child]
    pub apps_list_box: TemplateChild<gtk::ListBox>,
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
        klass.bind_template_callbacks();
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

        // Populate apps list with all applications
        self.populate_apps_list();

        // Set up filtering
        self.setup_filtering();
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

    pub fn populate_apps_list(&self) {
        let desktop_manager = self.desktop_manager.borrow();
        let entries = desktop_manager.get_entries();

        // Clear existing children
        while let Some(child) = self.apps_list_box.first_child() {
            self.apps_list_box.remove(&child);
        }

        // Add a row for each application
        for entry in entries {
            let label = Label::new(Some(&entry.name));
            label.set_halign(gtk::Align::Start);
            label.set_margin_start(12);
            label.set_margin_end(12);
            label.set_margin_top(8);
            label.set_margin_bottom(8);

            self.apps_list_box.append(&label);
        }
    }

    pub fn setup_filtering(&self) {
        let filter_entry = self.filter_entry.clone();

        // Set up filter function for the list box
        self.apps_list_box.set_filter_func(move |row| {
            let filter_text = filter_entry.text();

            // If no filter text, show all items
            if filter_text.is_empty() {
                println!("Empty filter text, showing all");
                return true;
            }

            // Get the label from the row
            if let Some(child) = row.child() {
                if let Ok(label) = child.downcast::<Label>() {
                    let app_name = label.text();
                    let matcher = SkimMatcherV2::default();
                    let matches = matcher.fuzzy_match(&app_name, &filter_text).is_some();

                    println!("Filter function called with text: '{}'", filter_text);
                    println!("App '{}' matches '{}': {}", app_name, filter_text, matches);

                    return matches;
                }
            }

            false
        });

        // Connect to entry changes to invalidate filter
        let apps_list_box = self.apps_list_box.clone();
        self.filter_entry.connect_changed(move |entry| {
            let filter_text = entry.text();
            println!("Filter text changed to: '{}'", filter_text);
            println!("invalidate_filter() called");
            apps_list_box.invalidate_filter();
            println!("Calling invalidate_filter()");
        });
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn update_apps_list(&self) {
        println!("update_apps_list callback triggered");
        self.apps_list_box.invalidate_filter();
    }
}
