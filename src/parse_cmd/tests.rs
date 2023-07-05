#[cfg(test)]
pub mod tests_parse_cmd {
    use crate::parse_cmd;

    #[test]
    fn cmd_should_be_parsed() {
        let args = vec![
            "openssl-execute".to_string(),
            "before=10d".to_string(),
            "ca=/cert/ca_certificate.pem".to_string(),
            "./hello.sh".to_string(),
        ];

        let cmd = parse_cmd::parse(args).unwrap();

        assert_eq!(10, cmd.before);
        assert_eq!("/cert/ca_certificate.pem", cmd.ca);
        assert_eq!("./hello.sh", cmd.cmd);
    }

    #[test]
    fn cmd_without_script_arg_should_fail() -> Result<(), String> {
        let args = vec![
            "openssl-execute".to_string(),
            "before=10d".to_string(),
            "ca=/cert/ca_certificate.pem".to_string(),
        ];

        match parse_cmd::parse(args) {
            Ok(_) => Err("it should be impossible to parse correctly the command line".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn should_not_parse_the_cmd() -> Result<(), String> {
        let args = vec!["openssl-execute".to_string()];

        match parse_cmd::parse(args) {
            Ok(_) => Err("it should be impossible to parse correctly the command line".to_string()),
            Err(_) => Ok(()),
        }
    }
}
