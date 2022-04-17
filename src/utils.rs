use crate::Cmd;
use std::{
    io,
    process::{Command, Stdio},
};

/// Checks for neovim and vim on path, returning a [`Cmd`] for one of them
/// if found, or an [`io::Error`] if neither is available
pub fn find_cmd() -> io::Result<Cmd> {
    if has_nvim_on_path() {
        Ok(Cmd::Neovim)
    } else if has_vim_on_path() {
        Ok(Cmd::Vim)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No vim or neovim instance found in path",
        ))
    }
}

/// Returns true if able to spawn a vim process
pub fn has_vim_on_path() -> bool {
    has_on_path("vim")
}

/// Returns true if able to spawn an nvim process
pub fn has_nvim_on_path() -> bool {
    has_on_path("nvim")
}

fn has_on_path(cmd: &str) -> bool {
    !matches!(
        Command::new(cmd)
            .arg("--help")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        Err(x) if x.kind() == io::ErrorKind::NotFound
    )
}
