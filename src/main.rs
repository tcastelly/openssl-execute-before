use chrono::offset::LocalResult;
use chrono::prelude::*;
use chrono::Utc;
use regex::Regex;
use std::thread;
use std::time;
use std::{env, process};

#[derive(Debug, Clone)]
struct Cmd {
    // number of days
    before: u32,
    ca: String,
    cmd: String,
}

impl Cmd {
    fn new(cmd: String) -> Cmd {
        Cmd {
            before: 0,
            ca: "".to_string(),
            cmd,
        }
    }

    fn merge(self, other: Cmd) -> Self {
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

fn get_month_num(month: &str) -> Option<u32> {
    [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, m)| {
        if m == &month {
            Some((i + 1) as u32)
        } else {
            None
        }
    })
}

fn get_expiration_cert(ca: &str) -> String {
    let output = process::Command::new("openssl")
        .arg("x509")
        .arg("-enddate")
        .arg("-noout")
        .arg("--in")
        .arg(ca)
        .output()
        .expect("failed to retrieve the expiration date of the `ca_certificate.pem` file");

    let output_str = String::from_utf8(output.stdout).unwrap();

    output_str.trim().to_string()
}

fn str_to_dt(dt: &str) -> LocalResult<DateTime<Utc>> {
    let re = Regex::new(
        r"^notAfter=([A-Z][a-z]{2}) ([0-9 ]{1,2}) ([0-9]{2}):([0-9]{2}):([0-9]{2}) ([0-9]{4}) GMT$",
    )
    .unwrap();

    let caps = re.captures(dt).unwrap();

    let month = get_month_num(caps[1].trim()).unwrap();

    let day = &caps[2].trim();
    let day = day.parse::<u32>().unwrap();

    let hour = &caps[3].trim();
    let hour = hour.parse::<u32>().unwrap();

    let min = &caps[4].trim();
    let min = min.parse::<u32>().unwrap();

    let sec = &caps[5].trim();
    let sec = sec.parse::<u32>().unwrap();

    let year = &caps[6].trim();
    let year = year.parse::<i32>().unwrap();

    Utc.with_ymd_and_hms(year, month, day, hour, min, sec)
}

fn retrieve_first_match(reg: &Regex, arg: String) -> Option<String> {
    let caps_opt = reg.captures(&arg);

    let capt_opt = if let Some(before_cap) = caps_opt {
        before_cap.get(1)
    } else {
        None
    };

    capt_opt.map(|str| str.as_str().to_string())
}

/// return the number of day as u32
/// e.g: before=2d
fn retrieve_before(cur_args: Cmd, arg: String) -> Cmd {
    let before_re = Regex::new(r"^before=([0-9]+)d$").unwrap();
    let before_match_opt = retrieve_first_match(&before_re, arg);

    if let Some(before_str) = before_match_opt {
        Cmd {
            before: before_str.parse().unwrap(),
            ..cur_args
        }
    } else {
        cur_args
    }
}

/// retrieve the ca path
/// e.g: ca=/cert/ca_certificate.pem
fn retrieve_ca(cur_args: Cmd, arg: String) -> Cmd {
    let ca_re = Regex::new(r"^ca=(.*)+$").unwrap();
    let ca_match_opt = retrieve_first_match(&ca_re, arg);

    if let Some(ca) = ca_match_opt {
        Cmd { ca, ..cur_args }
    } else {
        cur_args
    }
}

/// retrieve Cmd with all fields
fn parse_cmd(args: Vec<String>) -> Result<Cmd, String> {
    let file_or_cmd = match args.as_slice() {
        [.., last] => last.to_string(),
        _ => return Err("a minimum of one parameter is required".to_string()),
    };

    let cmd = args.into_iter().fold(Cmd::new(file_or_cmd), |cur, next| {
        let with_before = retrieve_before(cur.clone(), next.clone());
        let with_ca = retrieve_ca(cur, next);

        with_before.merge(with_ca)
    });

    Ok(cmd)
}

/// execute the external command
fn execute_external_cmd(cmd: String) {
    process::Command::new(cmd)
        .spawn()
        .expect("impossible to execute the file");
}

/// launch the command only once before expiration day
fn try_launch_cmd(cmd: &Cmd, diff_num_days: i64) {
    if cmd.before as i64 >= diff_num_days {
        println!("{} days exceeded, execute the command", diff_num_days);
        execute_external_cmd(cmd.cmd.to_string());
    } else {
        println!(
            "launch the arg script {} days before expiration",
            cmd.before
        );
    }
}

fn main() -> Result<(), String> {
    let cmd = parse_cmd(env::args().collect())?;

    let output_str = get_expiration_cert(&cmd.ca);

    match &output_str[..] {
        "" => Err("invalid certificate expiration date".to_string()),
        output_str => {
            let dt = str_to_dt(output_str);

            // calculate number of day between now and the expiration date
            let diff_num_days = (dt.unwrap() - Utc::now()).num_days();

            println!("ca will expire in {} days", diff_num_days);
            try_launch_cmd(&cmd, diff_num_days);

            let one_day = 60 * 60 * 24;
            loop {
                println!("next try in one day");
                thread::sleep(time::Duration::from_secs(one_day));

                try_launch_cmd(&cmd, diff_num_days);
            }
        }
    }
}
