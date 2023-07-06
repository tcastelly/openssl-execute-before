use crate::{openssl, openssl::prelude::*, parse_usr_cmd, process, process::prelude::*};
use chrono::Utc;
use std::rc::Rc;
use std::thread;
use std::time;

static ONE_DAY: u64 = 60 * 60 * 24;

/// launch the command only once before expiration day
fn try_launch_cmd(
    openssl_wrapper: Rc<dyn OpenSSHCmd + 'static>,
    process: Rc<dyn Process + 'static>,
    cmd: &parse_usr_cmd::Cmd,
) -> Result<(), String> {
    let dt = openssl_wrapper.get_expiration_cert(&cmd.ca).parse()?;

    // calculate number of day between now and the expiration date
    let diff_num_days = (dt.unwrap() - Utc::now()).num_days();
    println!("ca will expire in {} days", diff_num_days);

    if cmd.before as i64 >= diff_num_days {
        println!("{} days exceeded, execute the command", diff_num_days);
        process.execute_script(cmd);
    } else {
        println!(
            "launch the arg script {} days before the expiration date",
            cmd.before
        );
    }

    Ok(())
}

pub fn run(cli_args: Vec<String>) -> Result<(), String> {
    let cmd = parse_usr_cmd::parse(cli_args)?;

    let openssl_wrapper = openssl::OpenSSLWrapper::new();
    let openssl_wrapper_rc = Rc::new(openssl_wrapper);

    let process = process::ProcessWrapper::new();
    let process_rc = Rc::new(process);

    loop {
        try_launch_cmd(openssl_wrapper_rc.clone(), process_rc.clone(), &cmd)?;

        println!("next try in one day");
        thread::sleep(time::Duration::from_secs(ONE_DAY));
    }
}
