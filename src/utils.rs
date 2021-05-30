use crate::Cmd;
use std::{
    io,
    path::PathBuf,
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

/// Performs search to find vimrc based on platform, returning first valid
/// vimrc found
///
/// ### Unix
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim`
/// * `~/.config/nvim/init.vim`
/// * `~/.vimrc`
/// * `~/.vim/vimrc`
///
/// ### Windows
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim`
/// * `~/AppData/Local/nvim/init.vim`
/// * `~/_vimrc`
/// * `~/vimfiles/vimrc`
/// * `$VIM/_vimrc`
///
/// ### Other
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim`
pub fn find_vimrc() -> Option<PathBuf> {
    let xdg_config_home = shellexpand::env("$XDG_CONFIG_HOME");

    if cfg!(unix) {
        let home = shellexpand::tilde("~");
        let path1 = xdg_config_home.map(|home| {
            [home.as_ref(), "nvim", "init.vim"]
                .iter()
                .collect::<PathBuf>()
        });
        let path2 = [home.as_ref(), ".config", "nvim", "init.vim"]
            .iter()
            .collect::<PathBuf>();
        let path3 = [home.as_ref(), ".vimrc"].iter().collect::<PathBuf>();
        let path4 = [home.as_ref(), ".vim", "vimrc"].iter().collect::<PathBuf>();

        match (path1, path2, path3, path4) {
            (Ok(path), _, _, _) if path.exists() => Some(path),
            (_, path, _, _) if path.exists() => Some(path),
            (_, _, path, _) if path.exists() => Some(path),
            (_, _, _, path) if path.exists() => Some(path),
            _ => None,
        }
    } else if cfg!(windows) {
        let home = shellexpand::tilde("~");
        let vim_env = shellexpand::env("$VIM");

        let path1 = xdg_config_home.map(|home| {
            [home.as_ref(), "nvim", "init.vim"]
                .iter()
                .collect::<PathBuf>()
        });
        let path2 = [home.as_ref(), "AppData", "Local", "nvim", "init.vim"]
            .iter()
            .collect::<PathBuf>();
        let path3 = [home.as_ref(), "_vimrc"].iter().collect::<PathBuf>();
        let path4 = [home.as_ref(), "vimfiles", "vimrc"]
            .iter()
            .collect::<PathBuf>();
        let path5 = vim_env.map(|vim| [vim.as_ref(), "_vimrc"].iter().collect::<PathBuf>());

        match (path1, path2, path3, path4, path5) {
            (Ok(path), _, _, _, _) if path.exists() => Some(path),
            (_, path, _, _, _) if path.exists() => Some(path),
            (_, _, path, _, _) if path.exists() => Some(path),
            (_, _, _, path, _) if path.exists() => Some(path),
            (_, _, _, _, Ok(path)) if path.exists() => Some(path),
            _ => None,
        }
    } else {
        None
    }
}
