mod openssl;
mod parse_usr_cmd;
mod process;
use std::env;
mod try_run_on_expired;

fn main() -> Result<(), String> {
    try_run_on_expired::run(env::args().collect())?;

    Ok(())
}
