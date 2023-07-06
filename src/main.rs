mod openssl;
mod parse_usr_cmd;
mod process;
mod try_run_on_expired;

fn main() -> Result<(), String> {
    try_run_on_expired::run()?;

    Ok(())
}
