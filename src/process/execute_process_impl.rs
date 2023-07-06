use crate::parse_usr_cmd;
use crate::process::execute_process_io;
use std::process;

pub struct ProcessWrapper;

impl ProcessWrapper {
    pub fn new() -> ProcessWrapper {
        ProcessWrapper {}
    }
}

impl execute_process_io::Process for ProcessWrapper {
    fn execute_script(&self, cmd_args: &parse_usr_cmd::Cmd) {
        println!("execute arg script ...");
        let output = process::Command::new(&cmd_args.cmd)
            .output()
            .expect("impossible to execute the file");

        let output_str = match String::from_utf8(output.stdout) {
            Ok(output_str) => output_str,
            _ => "".to_string(),
        };

        if !output_str.is_empty() {
            println!();
            println!("output:");
            println!("{}", output_str);
        }
        println!("arg script executed with success");
    }
}
