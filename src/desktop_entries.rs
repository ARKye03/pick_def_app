use freedesktop_desktop_entry::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub icon: Option<String>,
    pub exec: String,
    pub mimetypes: Vec<String>,
    pub categories: Vec<String>,
    pub path: PathBuf,
}

pub struct DesktopEntryManager {
    entries: HashMap<String, AppEntry>,
}

impl DesktopEntryManager {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load_entries(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let locales = get_languages_from_env();

        let entries = Iter::new(default_paths())
            .entries(Some(&locales))
            .collect::<Vec<_>>();

        // Use HashMap to automatically handle duplicates - last entry wins
        for entry in entries {
            if let Some(app_entry) = self.parse_entry(entry) {
                // Use the desktop file name (without .desktop extension) as key
                let key = app_entry
                    .path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&app_entry.name)
                    .to_string();

                // Temporarily, skip apps with no associated mimetypes, maybe in a future customise which app can open what?
                if app_entry.mimetypes.is_empty() {
                    continue;
                } else {
                    self.entries.insert(key, app_entry);
                }
            }
        }

        Ok(())
    }

    pub fn get_entries(&self) -> Vec<&AppEntry> {
        self.entries.values().collect()
    }

    pub fn get_entries_for_mimetype(&self, mimetype: &str) -> Vec<&AppEntry> {
        self.entries
            .values()
            .filter(|entry| entry.mimetypes.contains(&mimetype.to_string()))
            .collect()
    }

    pub fn search_entries(&self, query: &str) -> Vec<&AppEntry> {
        let query = query.to_lowercase();
        self.entries
            .values()
            .filter(|entry| {
                entry.name.to_lowercase().contains(&query)
                    || entry
                        .categories
                        .iter()
                        .any(|cat| cat.to_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn get_all_categories(&self) -> Vec<String> {
        use std::collections::HashSet;

        let mut categories = HashSet::new();
        for entry in self.entries.values() {
            for category in &entry.categories {
                if !category.is_empty() {
                    categories.insert(category.clone());
                }
            }
        }

        let mut sorted_categories: Vec<String> = categories.into_iter().collect();
        sorted_categories.sort();
        sorted_categories
    }

    pub fn get_all_mimetypes(&self) -> Vec<String> {
        use std::collections::HashSet;

        let mut mimetypes = HashSet::new();
        for entry in self.entries.values() {
            for mimetype in &entry.mimetypes {
                if !mimetype.is_empty() {
                    mimetypes.insert(mimetype.clone());
                }
            }
        }

        let mut sorted_mimetypes: Vec<String> = mimetypes.into_iter().collect();
        sorted_mimetypes.sort();
        sorted_mimetypes
    }

    pub fn get_main_mimetype_categories(&self) -> Vec<String> {
        use std::collections::HashSet;

        let mut main_types = HashSet::new();
        for entry in self.entries.values() {
            for mimetype in &entry.mimetypes {
                if !mimetype.is_empty() {
                    // Extract the main type (part before the slash)
                    if let Some(main_type) = mimetype.split('/').next()
                        && !main_type.is_empty()
                    {
                        main_types.insert(main_type.to_string());
                    }
                }
            }
        }

        let mut sorted_main_types: Vec<String> = main_types.into_iter().collect();
        sorted_main_types.sort();
        sorted_main_types
    }
    fn parse_entry(&self, entry: DesktopEntry) -> Option<AppEntry> {
        let empty_locales: &[String] = &[];
        let name = entry.name(empty_locales)?.to_string();
        let icon = entry.icon().map(|s| s.to_string());
        let exec = entry.exec()?.to_string();

        let mimetypes = entry
            .mime_type()
            .map(|mt| mt.iter().map(|s| s.to_string()).collect::<Vec<String>>())
            .unwrap_or_default();

        let categories = entry
            .categories()
            .map(|cat| cat.iter().map(|s| s.to_string()).collect::<Vec<String>>())
            .unwrap_or_default();

        Some(AppEntry {
            name,
            icon,
            exec,
            mimetypes,
            categories,
            path: entry.path.clone(),
        })
    }
}
impl Default for DesktopEntryManager {
    fn default() -> Self {
        Self::new()
    }
}
