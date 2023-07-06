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
        process.execute_script(cmd)
    } else {
        println!(
            "launch the arg script {} days before the expiration date",
            cmd.before
        );

        Ok(())
    }
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

#[cfg(test)]
pub mod tests_launch_cmd {
    use crate::openssl::prelude::{MockOpenSSHCmd, Parse};
    use crate::parse_usr_cmd;
    use crate::process::prelude::MockProcess;
    use crate::try_run_on_expired::try_launch_cmd;
    use chrono::{DateTime, Datelike, LocalResult, TimeZone, Utc};
    use std::rc::Rc;

    struct FakeParse {
        dt: LocalResult<DateTime<Utc>>,
    }

    impl Parse for FakeParse {
        fn parse(&self) -> Result<LocalResult<DateTime<Utc>>, String> {
            Ok(self.dt)
        }
    }

    impl FakeParse {
        fn new(dt: LocalResult<DateTime<Utc>>) -> FakeParse {
            FakeParse { dt }
        }
    }

    #[test]
    fn should_exec_a_script() {
        let now = Utc::now();
        let now_plus_5d = Utc.with_ymd_and_hms(now.year(), now.month(), now.day() + 5, 0, 0, 0);

        let mut cmd = parse_usr_cmd::Cmd::new("ca=/cert/certificate_ca.pem".to_string());
        cmd.before = 10;

        let mut mock_process = MockProcess::new();
        mock_process
            .expect_execute_script()
            .once()
            .returning(|_x| Ok(()));

        // save the `mut` mock in a new variable
        let mock_process = mock_process;
        let mock_process_rc = Rc::new(mock_process);

        let mut mock_openssl_wrapper = MockOpenSSHCmd::new();
        mock_openssl_wrapper
            .expect_get_expiration_cert()
            .once()
            .returning(move |_x| Box::new(FakeParse::new(now_plus_5d)));

        // save the `mut` mock in a new variable
        let mock_openssl_wrapper = mock_openssl_wrapper;
        let mock_openssl_rc = Rc::new(mock_openssl_wrapper);

        try_launch_cmd(mock_openssl_rc, mock_process_rc, &cmd).unwrap();
    }

    #[test]
    fn should_not_exec_a_script() {
        let mut cmd = parse_usr_cmd::Cmd::new("ca=/cert/certificate_ca.pem".to_string());
        cmd.before = 10;

        let now = Utc::now();
        let now_plus_11d =
            Utc.with_ymd_and_hms(now.year(), now.month(), now.day() + cmd.before + 2, 0, 0, 0);

        let mut mock_process = MockProcess::new();
        mock_process
            .expect_execute_script()
            .times(0)
            .returning(|_x| Err("should not be called".to_string()));

        // save the `mut` mock in a new variable
        let mock_process = mock_process;
        let mock_process_rc = Rc::new(mock_process);

        let mut mock_openssl_wrapper = MockOpenSSHCmd::new();
        mock_openssl_wrapper
            .expect_get_expiration_cert()
            .once()
            .returning(move |_x| Box::new(FakeParse::new(now_plus_11d)));

        // save the `mut` mock in a new variable
        let mock_openssl_wrapper = mock_openssl_wrapper;
        let mock_openssl_rc = Rc::new(mock_openssl_wrapper);

        try_launch_cmd(mock_openssl_rc, mock_process_rc, &cmd).unwrap();
    }
}
