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
        process::Command::new(&cmd_args.cmd)
            .spawn()
            .expect("impossible to execute the file");
    }
}
