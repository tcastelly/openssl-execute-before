use chrono::offset::LocalResult;
use chrono::prelude::*;
use regex::Regex;
use std::process;

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
        .arg("/Users/tcy/Documents/dev/workspaces/core/poc-broadcast/rabbitmq/broadcast-producer/cert/ca_certificate.pem")
        .output()
        .expect("failed to retrieve the expiration date of the `ca_certificate.pem` file");

    let output_str = String::from_utf8(outout.stdout).unwrap();

    output_str.trim().to_string()
}

fn str_to_dt(dt: &str) -> LocalResult<DateTime<Utc>> {
    let re = Regex::new(
        r"notAfter=([A-Z][a-z]{2}) ([0-9 ]{1,2}) ([0-9]{2}):([0-9]{2}):([0-9]{2}) ([0-9]{4}) GMT$",
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

fn main() {
    // from command line retrieve the date to launch a command
    // e.g before=2d

    // retrieve the real expirated date with openssl

    // retrieve the command to execute

    let output_str = get_expiration_cert();
    println!("capture: {:?}", output_str);

    let dt = str_to_dt(&output_str);
    println!("{:?}", dt);
}
