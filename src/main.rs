mod openssl;
mod parse_usr_cmd;
mod process;
mod try_launch_cmd;

use openssl::prelude::*;

use std::env;

fn main() -> Result<(), String> {
    let cmd = parse_usr_cmd::parse(env::args().collect())?;

    let dt = openssl::OpenSSLWrapper::new()
        .get_expiration_cert(&cmd.ca)
        .parse()?;

    try_launch_cmd::run(cmd, dt);

    Ok(())
}
