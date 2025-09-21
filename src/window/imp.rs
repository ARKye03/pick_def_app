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
    #[template_child]
    pub app_mime_types_list_box: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub mime_types_stack: TemplateChild<gtk::Stack>,
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

        // Set up app selection handler
        self.setup_app_selection();
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
            button.set_active(true);

            // It's so huge, it doesn't convince me
            // button.add_css_class("round");

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

            // Use the label text itself to identify the app
            self.apps_list_box.append(&label);
        }
    }

    pub fn populate_app_mimetypes(&self, app_name: &str) {
        let desktop_manager = self.desktop_manager.borrow();
        let entries = desktop_manager.get_entries();

        // Clear existing children
        while let Some(child) = self.app_mime_types_list_box.first_child() {
            self.app_mime_types_list_box.remove(&child);
        }

        // Find the selected app and show its mimetypes
        if let Some(app_entry) = entries.iter().find(|entry| entry.name == app_name) {
            if app_entry.mimetypes.is_empty() {
                // App selected but no mimetypes - show no_mime_types_page
                self.mime_types_stack
                    .set_visible_child_name("no_mime_types_page");
            } else {
                // App has mimetypes - populate list and show list page
                for mimetype in &app_entry.mimetypes {
                    let label = Label::new(Some(mimetype));
                    label.set_halign(gtk::Align::Start);
                    label.set_margin_start(12);
                    label.set_margin_end(12);
                    label.set_margin_top(8);
                    label.set_margin_bottom(8);

                    self.app_mime_types_list_box.append(&label);
                }
                self.mime_types_stack
                    .set_visible_child_name("app_mime_types_list_box_page");
            }
        }
    }

    pub fn setup_filtering(&self) {
        let filter_entry = self.filter_entry.clone();

        // Set up filter function for the list box
        self.apps_list_box.set_filter_func(move |row| {
            let filter_text = filter_entry.text();

            // If no filter text, show all items
            if filter_text.is_empty() {
                return true;
            }

            // Get the label from the row
            if let Some(child) = row.child()
                && let Ok(label) = child.downcast::<Label>()
            {
                let app_name = label.text();
                let matcher = SkimMatcherV2::default();
                let matches = matcher.fuzzy_match(&app_name, &filter_text).is_some();
                return matches;
            }

            false
        });
    }

    pub fn setup_app_selection(&self) {
        let obj = self.obj();
        let obj_weak = obj.downgrade();

        self.apps_list_box.connect_row_selected(move |_, row| {
            if let Some(obj) = obj_weak.upgrade() {
                let imp = obj.imp();
                if let Some(row) = row {
                    if let Some(child) = row.child()
                        && let Ok(label) = child.downcast::<Label>()
                    {
                        let app_name = label.text();
                        imp.populate_app_mimetypes(&app_name);
                    }
                } else {
                    // No row selected - clear mimetypes and show no_app_selected_page
                    while let Some(child) = imp.app_mime_types_list_box.first_child() {
                        imp.app_mime_types_list_box.remove(&child);
                    }
                    imp.mime_types_stack
                        .set_visible_child_name("no_app_selected_page");
                }
            }
        });
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn update_apps_list(&self) {
        self.apps_list_box.invalidate_filter();
    }
}
