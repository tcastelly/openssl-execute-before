use chrono::offset::LocalResult;
use chrono::prelude::*;
use regex::Regex;
use std::{env, process};

#[derive(Debug, Clone, Copy)]
struct Cmd<'a> {
    // number of days
    before: u32,
    ca: &'a str,
    cmd: &'a str,
}

impl<'a> Cmd<'a> {
    fn new() -> Cmd<'a> {
        Cmd {
            before: 0,
            ca: "",
            cmd: "",
        }
    }

    fn merge(self, other: Cmd<'a>) -> Self {
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

fn get_expiration_cert() -> String {
    let outout = process::Command::new("openssl")
        .arg("x509")
        .arg("-enddate")
        .arg("-noout")
        .arg("--in")
        .arg("/home/tcy/documents/dev/workspaces/core/broadcast-poc/rabbitmq/broadcast-producer/cert/ca_certificate.pem")
        .output()
        .expect("failed to retrieve the expiration date of the `ca_certificate.pem` file");

    let output_str = String::from_utf8(outout.stdout).unwrap();

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

fn retrieve_first_match<'a, 'b>(reg: &'a Regex, arg: &'b str) -> Option<&'b str> {
    let caps_opt = reg.captures(arg);

    let capt_opt = if let Some(before_cap) = caps_opt {
        before_cap.get(1)
    } else {
        None
    };

    if let Some(str) = capt_opt {
        Some(str.as_str())
    } else {
        None
    }
}

fn retrieve_before<'a, 'b>(cur_args: &'a Cmd<'b>, arg: &'a str) -> Cmd<'b> {
    let before_re = Regex::new(r"^before=([0-9])d$").unwrap();
    let before_match_opt = retrieve_first_match(&before_re, arg);

    if let Some(before_str) = before_match_opt {
        Cmd {
            before: before_str.parse().unwrap(),
            ..*cur_args
        }
    } else {
        Cmd { ..*cur_args }
    }
}

fn retrieve_ca<'a, 'b>(cur_args: &'a Cmd<'b>, arg: &'a str) -> Cmd<'a> {
    let ca_re = Regex::new(r"^ca=(.*)+$").unwrap();
    let ca_match_opt = retrieve_first_match(&ca_re, arg);

    if let Some(ca) = ca_match_opt {
        Cmd { ca, ..*cur_args }
    } else {
        Cmd { ..*cur_args }
    }
}

fn main() {
    // from command line retrieve the date to launch a command

    let args = env::args().skip(1);

    let cmd: Cmd = args.fold(Cmd::new(), |cur, next| {
        let with_before = retrieve_before(&cur, &next);
        let with_ca = retrieve_ca(&cur, &next);

        with_ca

        // with_before.merge(with_ca)
    });

    // e.g before=2d
    println!("{:?}", cmd);

    // retrieve the real expirated date with openssl

    // retrieve the command to openssl-execute

    let output_str = get_expiration_cert();
    println!("capture: {:?}", output_str);

    if !output_str.is_empty() {
        let dt = str_to_dt(&output_str);
        println!("{:?}", dt);
    }
}
