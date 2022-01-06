use std::fs::{self, DirEntry};

fn main() {
    match fs::read_dir("/sys/class/power_supply") {
        Ok(bat_paths) => {
            for bat_path in bat_paths {
                match bat_path {
                    Ok(entry) => {
                        // Battery::from(&entry).display();
                        let power_supply = Battery::from(entry);
                        match power_supply {
                            Some(battery) => {
                                battery.display();
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    };
}

struct Battery {
    name: String,
    capacity: String,
}

impl Battery {
    fn from(path: DirEntry) -> Option<Self> {
        // println!("{:?}", path);
        match fs::read_dir(path.path()) {
            Ok(entry) => {
                let mut out: Option<Self> = None;
                for e in entry {
                    let entry = e.unwrap();
                    match entry.path().file_name().unwrap().to_str().unwrap() {
                        "capacity" => {
                            // println!("{:?}", String::from_utf8(fs::read(entry.path()).unwrap()));
                            out = Some(Battery {
                                name: path.file_name().to_str().unwrap().into(),
                                capacity: String::from_utf8(fs::read(entry.path()).unwrap())
                                    .unwrap()
                                    .trim_end_matches('\n')
                                    .into(),
                            });
                        }
                        _ => {}
                    };
                }
                out
            }
            _ => None,
        }
    }

    fn display(&self) {
        println!("{}: {}%", self.name, self.capacity);
    }
}
