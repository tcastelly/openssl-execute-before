use self::super::super::parse_usr_cmd;

pub trait Process {
    /// execute the external command
    fn execute_script(&self, cmd_args: &parse_usr_cmd::Cmd);
}
