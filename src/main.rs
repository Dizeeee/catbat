use core::fmt;
use std::{
    fs::{self, DirEntry},
    process,
};

fn main() {
    let power_supplies = fs::read_dir("/sys/class/power_supply");
    let mut found_bat = false;

    for ps_path in power_supplies.into_iter().flatten().flatten() {
        if let Some(battery) = Battery::load(ps_path) {
            found_bat = true;
            println!("{}", battery);
        }
    }

    if !found_bat {
        eprintln!("No batteries found!");
        process::exit(1);
    }
}

struct Battery {
    name: String,
    capacity: String,
    manufacturer: String,
    model_name: String,
}

impl Battery {
    fn load(path: DirEntry) -> Option<Self> {
        Some(Self {
            name: path.file_name().to_str()?.into(),
            capacity: fs::read_to_string(path.path().join("capacity"))
                .ok()?
                .trim_end_matches('\n')
                .into(),
            manufacturer: fs::read_to_string(path.path().join("manufacturer"))
                .unwrap_or_default()
                .trim_end_matches('\n')
                .into(),
            model_name: fs::read_to_string(path.path().join("model_name"))
                .unwrap_or_default()
                .trim_end_matches('\n')
                .into(),
        })
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}% - {}",
            self.name,
            self.capacity,
            format!("{} {}", self.manufacturer, self.model_name).trim()
        )
    }
}
