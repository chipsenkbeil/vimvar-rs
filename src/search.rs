use std::{borrow::Cow, path::PathBuf};

/// Input configuration for finding vimrc
trait FindVimrcConfig {
    type Err;

    /// Returns $XDG_CONFIG_HOME path if possible
    fn xdg_config_home(&self) -> Result<Cow<'static, str>, Self::Err>;

    /// Returns $HOME path
    fn home(&self) -> Cow<'static, str>;

    /// Returns $VIM_ENV path if possible
    fn vim_env(&self) -> Result<Cow<'static, str>, Self::Err>;
}

/// Standard implementation of input for find vimrc config
struct StandardFindVimrcConfig;

impl FindVimrcConfig for StandardFindVimrcConfig {
    type Err = shellexpand::LookupError<std::env::VarError>;

    fn xdg_config_home(&self) -> Result<Cow<'static, str>, Self::Err> {
        shellexpand::env("$XDG_CONFIG_HOME")
    }

    fn home(&self) -> Cow<'static, str> {
        shellexpand::tilde("~")
    }

    fn vim_env(&self) -> Result<Cow<'static, str>, Self::Err> {
        shellexpand::env("$VIM")
    }
}

/// Performs search to find vimrc based on platform, returning first valid
/// vimrc found. Will check for both `init.vim` and `init.lua` file types.
///
/// ### Unix
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim` (or `init.lua`)
/// * `~/.config/nvim/init.vim` (or `init.lua`)
/// * `~/.vimrc`
/// * `~/.vim/vimrc`
///
/// ### Windows
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim` (or `init.lua`)
/// * `~/AppData/Local/nvim/init.vim` (or `init.lua`)
/// * `~/_vimrc`
/// * `~/vimfiles/vimrc`
/// * `$VIM/_vimrc`
///
/// ### Other
///
/// Looks for a vimrc configuration file in the following places:
///
/// * `$XDG_CONFIG_HOME/nvim/init.vim` (or `init.lua`)
pub fn find_vimrc() -> Option<PathBuf> {
    find_vimrc_impl(StandardFindVimrcConfig)
}

fn find_vimrc_impl<C>(config: C) -> Option<PathBuf>
where
    C: FindVimrcConfig,
{
    let xdg_config_home = config.xdg_config_home();

    if cfg!(unix) {
        let home = config.home();

        vec![
            // $XDG_CONFIG_HOME/nvim/init.lua
            xdg_config_home
                .as_ref()
                .map(|home| {
                    [home.as_ref(), "nvim", "init.lua"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
            // $XDG_CONFIG_HOME/nvim/init.vim
            xdg_config_home
                .map(|home| {
                    [home.as_ref(), "nvim", "init.vim"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
            // $HOME/.config/nvim/init.lua
            Some(
                [home.as_ref(), ".config", "nvim", "init.lua"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
            // $HOME/.config/nvim/init.vim
            Some(
                [home.as_ref(), ".config", "nvim", "init.vim"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
            // $HOME/.vimrc
            Some([home.as_ref(), ".vimrc"].iter().collect::<PathBuf>()),
            // $HOME/.vim/.vimrc
            Some([home.as_ref(), ".vim", "vimrc"].iter().collect::<PathBuf>()),
        ]
        .into_iter()
        .find_map(|maybe_path| match maybe_path {
            Some(path) if path.exists() => Some(path),
            _ => None,
        })
    } else if cfg!(windows) {
        let home = config.home();
        let vim_env = config.vim_env();

        vec![
            // $XDG_CONFIG_HOME/nvim/init.lua
            xdg_config_home
                .as_ref()
                .map(|home| {
                    [home.as_ref(), "nvim", "init.lua"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
            // $XDG_CONFIG_HOME/nvim/init.vim
            xdg_config_home
                .map(|home| {
                    [home.as_ref(), "nvim", "init.vim"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
            // $HOME/AppData/Local/nvim/init.lua
            Some(
                [home.as_ref(), "AppData", "Local", "nvim", "init.lua"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
            // $HOME/AppData/Local/nvim/init.vim
            Some(
                [home.as_ref(), "AppData", "Local", "nvim", "init.vim"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
            // $HOME/_vimrc
            Some([home.as_ref(), "_vimrc"].iter().collect::<PathBuf>()),
            // $HOME/vimfiles/vimrc
            Some(
                [home.as_ref(), "vimfiles", "vimrc"]
                    .iter()
                    .collect::<PathBuf>(),
            ),
            // $VIM/_vimrc
            vim_env
                .map(|vim| [vim.as_ref(), "_vimrc"].iter().collect::<PathBuf>())
                .ok(),
        ]
        .into_iter()
        .find_map(|maybe_path| match maybe_path {
            Some(path) if path.exists() => Some(path),
            _ => None,
        })
    } else {
        vec![
            // $XDG_CONFIG_HOME/nvim/init.lua
            xdg_config_home
                .as_ref()
                .map(|home| {
                    [home.as_ref(), "nvim", "init.lua"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
            // $XDG_CONFIG_HOME/nvim/init.vim
            xdg_config_home
                .map(|home| {
                    [home.as_ref(), "nvim", "init.vim"]
                        .iter()
                        .collect::<PathBuf>()
                })
                .ok(),
        ]
        .into_iter()
        .find_map(|maybe_path| match maybe_path {
            Some(path) if path.exists() => Some(path),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, path::Path};
    use tempfile::tempdir;

    struct TestFindVimrcConfig {
        xdg_config_home: Result<Cow<'static, str>, shellexpand::LookupError<std::env::VarError>>,
        home: Cow<'static, str>,
        vim_env: Result<Cow<'static, str>, shellexpand::LookupError<std::env::VarError>>,
    }

    impl FindVimrcConfig for TestFindVimrcConfig {
        type Err = shellexpand::LookupError<std::env::VarError>;

        fn xdg_config_home(&self) -> Result<Cow<'static, str>, Self::Err> {
            self.xdg_config_home.clone()
        }

        fn home(&self) -> Cow<'static, str> {
            self.home.clone()
        }

        fn vim_env(&self) -> Result<Cow<'static, str>, Self::Err> {
            self.vim_env.clone()
        }
    }

    impl Default for TestFindVimrcConfig {
        /// Create a test config with fake, non-existing paths
        fn default() -> Self {
            Self {
                xdg_config_home: Ok(Cow::Owned(
                    tempdir()
                        .unwrap()
                        .into_path()
                        .to_string_lossy()
                        .into_owned(),
                )),
                home: Cow::Owned(
                    tempdir()
                        .unwrap()
                        .into_path()
                        .to_string_lossy()
                        .into_owned(),
                ),
                vim_env: Ok(Cow::Owned(
                    tempdir()
                        .unwrap()
                        .into_path()
                        .to_string_lossy()
                        .into_owned(),
                )),
            }
        }
    }

    fn create_file(root: impl AsRef<Path>, components: &[&str]) -> PathBuf {
        assert!(!components.is_empty(), "Missing components");
        let root = root.as_ref();
        let (last, components) = components.split_last().unwrap();

        let mut full_path = root.to_path_buf();
        for path in components {
            full_path = full_path.join(path);
        }

        std::fs::create_dir_all(full_path.as_path()).expect("Failed to create part of file path");
        let full_path = full_path.join(last);
        File::create(full_path.as_path()).expect("Failed to create file");
        full_path
    }

    fn tempdir_to_cow_str(t: &tempfile::TempDir) -> Cow<'static, str> {
        Cow::Owned(t.as_ref().to_string_lossy().into_owned())
    }

    #[test]
    #[cfg(unix)]
    fn find_vimrc_on_unix_should_succeed_if_available_at_home_config_nvim_init_lua() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &[".config", "nvim", "init.lua"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(unix)]
    fn find_vimrc_on_unix_should_succeed_if_available_at_home_config_nvim_init_vim() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &[".config", "nvim", "init.vim"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(unix)]
    fn find_vimrc_on_unix_should_succeed_if_available_at_home_vimrc() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &[".vimrc"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(unix)]
    fn find_vimrc_on_unix_should_succeed_if_available_at_home_vim_vimrc() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &[".vim", "vimrc"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(windows)]
    fn find_vimrc_on_windows_should_succeed_if_available_at_home_appdata_local_nvim_init_lua() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["AppData", "Local", "nvim", "init.lua"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(windows)]
    fn find_vimrc_on_windows_should_succeed_if_available_at_home_appdata_local_nvim_init_vim() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["AppData", "Local", "nvim", "init.vim"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(windows)]
    fn find_vimrc_on_windows_should_succeed_if_available_at_home_vimrc() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["_vimrc"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(windows)]
    fn find_vimrc_on_windows_should_succeed_if_available_at_home_vimfiles_vimrc() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["vimfiles", "vimrc"]);

        let config = TestFindVimrcConfig {
            home: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    #[cfg(windows)]
    fn find_vimrc_on_windows_should_succeed_if_available_at_vimenv_vimrc() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["_vimrc"]);

        let config = TestFindVimrcConfig {
            vim_env: tempdir_to_cow_str(&root),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    fn find_vimrc_should_succeed_if_available_at_xdg_nvim_init_lua() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["nvim", "init.lua"]);

        let config = TestFindVimrcConfig {
            xdg_config_home: Ok(tempdir_to_cow_str(&root)),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }

    #[test]
    fn find_vimrc_should_succeed_if_available_at_xdg_nvim_init_vim() {
        let root = tempdir().unwrap();
        let config_file = create_file(root.as_ref(), &["nvim", "init.vim"]);

        let config = TestFindVimrcConfig {
            xdg_config_home: Ok(tempdir_to_cow_str(&root)),
            ..Default::default()
        };

        assert_eq!(find_vimrc_impl(config), Some(config_file));
    }
}
