
const SETTINGS_FILENAME: &str = "settings.json";

struct Settings {
    workrave_historystats_path: Option<String>,
}

impl Settings {
    fn ui() {

    }

    fn settings_init() {
        // see if settings file exists
        // if not, create it
        // else, read it and overwrite current
    }

    fn load_settings() -> Result<Ok, Err>{

    }

    fn save_settings() -> Result<Ok, Err> {

    }
}
