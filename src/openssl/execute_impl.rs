use crate::openssl::execute_io;
use crate::openssl::execute_io::Parse;
use chrono::prelude::*;
use chrono::{DateTime, LocalResult, Utc};
use regex::Regex;
use std::process;

pub struct OpenSSLWrapper;

impl OpenSSLWrapper {
    pub fn new() -> OpenSSLWrapper {
        OpenSSLWrapper {}
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

struct Res {
    input: String,
}

impl Parse for Res {
    fn parse(&self) -> Result<LocalResult<DateTime<Utc>>, String> {
        if self.input.is_empty() {
            Err("invalid certificate".to_string())
        } else {
            Ok(str_to_dt(&self.input))
        }
    }
}

impl Res {
    fn new(input: String) -> Res {
        Res { input }
    }
}

impl execute_io::OpenSSHCmd for OpenSSLWrapper {
    fn get_expiration_cert(&self, ca: &str) -> Box<dyn Parse + 'static> {
        let output = process::Command::new("openssl")
            .arg("x509")
            .arg("-enddate")
            .arg("-noout")
            .arg("--in")
            .arg(ca)
            .output()
            .expect("failed to retrieve the expiration date of the `ca_certificate.pem` file");

        let output_str = String::from_utf8(output.stdout).unwrap();

        Box::new(Res::new(output_str.trim().to_string()))
    }
}
