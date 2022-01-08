use core::fmt;
use std::fs::{self, DirEntry};

fn main() {
    for bat_path in fs::read_dir("/sys/class/power_supply")
        .into_iter()
        .flatten()
        .flatten()
    {
        if let Some(battery) = Battery::load(bat_path) {
            println!("{}", battery);
        }
    }
}

struct Battery {
    name: String,
    capacity: String,
}

impl Battery {
    fn load(path: DirEntry) -> Option<Self> {
        // if let Ok(entry) = fs::read_dir(path.path()) {
        //     let mut out: Option<Self> = None;
        //     for e in entry.flatten() {
        //         if e.path().file_name().unwrap().to_str().unwrap() == "capacity" {
        //             out = Some(Battery {
        //                 name: path.file_name().to_str().unwrap().into(),
        //                 capacity: fs::read_to_string(e.path())
        //                     .unwrap()
        //                     .trim_end_matches('\n')
        //                     .into(),
        //             });
        //         };
        //     }
        //     out
        // } else {
        //     None
        // }
        Some(Self {
            name: path.file_name().to_str()?.into(),
            capacity: fs::read_to_string(path.path().join("capacity"))
                .ok()?
                .trim_end_matches('\n')
                .into(),
        })
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}%", self.name, self.capacity)
    }
}
