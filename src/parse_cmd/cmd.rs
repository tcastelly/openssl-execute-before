#[derive(Debug, Clone)]
pub struct Cmd {
    // number of days
    pub before: u32,

    // certificate file path
    pub ca: String,

    // command to launch "before"
    pub cmd: String,
}

impl Cmd {
    pub fn new(cmd: String) -> Cmd {
        Cmd {
            before: 0,
            ca: "".to_string(),
            cmd,
        }
    }

    pub fn merge(self, other: Cmd) -> Self {
        Self {
            before: match self.before {
                0 => other.before,
                _ => self.before,
            },
            ca: if self.ca.is_empty() {
                other.ca
            } else {
                self.ca
            },
            cmd: if self.cmd.is_empty() {
                other.cmd
            } else {
                self.cmd
            },
        }
    }
}
