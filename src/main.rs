use core::fmt;
use std::fs::{self, DirEntry};

fn main() {
    for bat_path in fs::read_dir("/sys/class/power_supply")
        .into_iter()
        .flatten()
        .flatten()
    {
        if let Some(battery) = Battery::from(bat_path) {
            println!("{}", battery);
        }
    }
}

struct Battery {
    name: String,
    capacity: String,
}

impl Battery {
    fn from(path: DirEntry) -> Option<Self> {
        if let Ok(entry) = fs::read_dir(path.path()) {
            let mut out: Option<Self> = None;
            for e in entry.flatten() {
                if e.path().file_name().unwrap().to_str().unwrap() == "capacity" {
                    out = Some(Battery {
                        name: path.file_name().to_str().unwrap().into(),
                        capacity: fs::read_to_string(e.path())
                            .unwrap()
                            .trim_end_matches('\n')
                            .into(),
                    });
                };
            }
            out
        } else {
            None
        }
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}%", self.name, self.capacity)
    }
}
