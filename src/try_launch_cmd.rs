use crate::process::prelude::*;
use crate::{parse_usr_cmd, process};
use chrono::{DateTime, LocalResult, Utc};
use std::rc::Rc;
use std::thread;
use std::time;

/// launch the command only once before expiration day
fn try_launch_cmd(
    process: Rc<dyn Process + 'static>,
    cmd: &parse_usr_cmd::Cmd,
    diff_num_days: i64,
) {
    if cmd.before as i64 >= diff_num_days {
        println!("{} days exceeded, execute the command", diff_num_days);
        process.execute_script(cmd);
    } else {
        println!(
            "launch the arg script {} days before expiration",
            cmd.before
        );
    }
}

pub fn run(cmd: parse_usr_cmd::Cmd, dt: LocalResult<DateTime<Utc>>) {
    let process = process::ProcessWrapper::new();
    let process_rc = Rc::new(process);

    // calculate number of day between now and the expiration date
    let diff_num_days = (dt.unwrap() - Utc::now()).num_days();

    println!("ca will expire in {} days", diff_num_days);
    try_launch_cmd(process_rc.clone(), &cmd, diff_num_days);

    let one_day = 60 * 60 * 24;
    loop {
        println!("next try in one day");
        thread::sleep(time::Duration::from_secs(one_day));

        // calculate number of day between now and the expiration date
        let diff_num_days = (dt.unwrap() - Utc::now()).num_days();

        try_launch_cmd(process_rc.clone(), &cmd, diff_num_days);
    }
}
