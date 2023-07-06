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
    let month_up = month.to_uppercase();
    [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, m)| {
        if m == &month_up {
            Some((i + 1) as u32)
        } else {
            None
        }
    })
}

fn str_to_dt(dt: &str) -> LocalResult<DateTime<Utc>> {
    let re = Regex::new(
        r"^notAfter=((?i)[a-z]{3}) ([0-9 ]{1,2}) ([0-9]{2}):([0-9]{2}):([0-9]{2}) ([0-9]{4}) GMT$",
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

#[test]
fn should_resolve_month_index() {
    let m = get_month_num("jan").unwrap();
    assert_eq!(m, 1);

    let m = get_month_num("feb").unwrap();
    assert_eq!(m, 2);

    let m = get_month_num("mar").unwrap();
    assert_eq!(m, 3);

    let m = get_month_num("apr").unwrap();
    assert_eq!(m, 4);

    let m = get_month_num("may").unwrap();
    assert_eq!(m, 5);

    let m = get_month_num("jun").unwrap();
    assert_eq!(m, 6);

    let m = get_month_num("jul").unwrap();
    assert_eq!(m, 7);

    let m = get_month_num("aug").unwrap();
    assert_eq!(m, 8);

    let m = get_month_num("sep").unwrap();
    assert_eq!(m, 9);

    let m = get_month_num("oct").unwrap();
    assert_eq!(m, 10);

    let m = get_month_num("nov").unwrap();
    assert_eq!(m, 11);

    let m = get_month_num("dec").unwrap();
    assert_eq!(m, 12);
}

#[test]
fn should_parse_openssl_res() {
    let d = str_to_dt("notAfter=Jul  2 09:30:45 2024 GMT").unwrap();
    assert_eq!(d.year(), 2024);
    assert_eq!(d.month(), 7);
    assert_eq!(d.day(), 2);
    assert_eq!(d.hour(), 9);
    assert_eq!(d.minute(), 30);
    assert_eq!(d.second(), 45);

    let d = str_to_dt("notAfter=Jul 22 09:30:45 2024 GMT").unwrap();
    assert_eq!(d.year(), 2024);
    assert_eq!(d.month(), 7);
    assert_eq!(d.day(), 22);
    assert_eq!(d.hour(), 9);
    assert_eq!(d.minute(), 30);
    assert_eq!(d.second(), 45);

    let d = str_to_dt("notAfter=jul 22 09:30:45 2024 GMT").unwrap();
    assert_eq!(d.year(), 2024);
    assert_eq!(d.month(), 7);
    assert_eq!(d.day(), 22);
    assert_eq!(d.hour(), 9);
    assert_eq!(d.minute(), 30);
    assert_eq!(d.second(), 45);

    let d = str_to_dt("notAfter=JUL 22 09:30:45 2024 GMT").unwrap();
    assert_eq!(d.year(), 2024);
    assert_eq!(d.month(), 7);
    assert_eq!(d.day(), 22);
    assert_eq!(d.hour(), 9);
    assert_eq!(d.minute(), 30);
    assert_eq!(d.second(), 45);
}
