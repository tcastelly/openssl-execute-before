use crate::parse_cmd::Cmd;
use regex::Regex;

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
pub fn parse(args: Vec<String>) -> Result<Cmd, String> {
    let min_param_err = Err("a minimum of one parameter is required".to_string());

    if args.len() == 1 {
        return min_param_err;
    }

    let file_or_cmd = match args.as_slice() {
        [.., last] => last.to_string(),
        _ => return min_param_err,
    };

    if file_or_cmd.contains('=') {
        return Err("the last parameters has to be the file to execute".to_string());
    }

    let cmd = args.into_iter().fold(Cmd::new(file_or_cmd), |cur, next| {
        let with_before = retrieve_before(cur.clone(), next.clone());
        let with_ca = retrieve_ca(cur, next);

        with_before.merge(with_ca)
    });

    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use crate::parse_cmd::parse::{retrieve_before, retrieve_ca, retrieve_first_match};
    use crate::parse_cmd::Cmd;
    use regex::Regex;

    #[test]
    fn should_retrieve_first_match() {
        let re = Regex::new(r"(hello) (world)").unwrap();
        let res = retrieve_first_match(&re, "hello world".to_string()).unwrap();

        assert_eq!("hello", res);
    }

    #[test]
    fn should_retrieve_before_2d() {
        let cmd = Cmd {
            before: 0,
            ca: "ca".to_string(),
            cmd: "cmd".to_string(),
        };

        let before = retrieve_before(cmd, "before=2d".to_string());

        assert_eq!("ca", before.ca);
        assert_eq!("cmd", before.cmd);
        assert_eq!(2, before.before);
    }

    #[test]
    fn should_retrieve_0d_if_no_before() {
        let cmd = Cmd {
            before: 0,
            ca: "ca".to_string(),
            cmd: "cmd".to_string(),
        };

        let before = retrieve_before(cmd, "".to_string());

        assert_eq!("ca", before.ca);
        assert_eq!("cmd", before.cmd);
        assert_eq!(0, before.before);
    }

    #[test]
    fn should_retrieve_ca() {
        let cmd = Cmd {
            before: 0,
            ca: "ca".to_string(),
            cmd: "cmd".to_string(),
        };

        let ca = retrieve_ca(cmd, "ca=/cert/ca.pem".to_string());
        assert_eq!("/cert/ca.pem", ca.ca);
        assert_eq!("cmd", ca.cmd);
        assert_eq!(0, ca.before);
    }

    #[test]
    fn should_retrieve_empty_ca() {
        let cmd = Cmd {
            before: 0,
            ca: "".to_string(),
            cmd: "cmd".to_string(),
        };

        let ca = retrieve_ca(cmd, "".to_string());
        assert_eq!("", ca.ca);
        assert_eq!("cmd", ca.cmd);
        assert_eq!(0, ca.before);
    }
}
