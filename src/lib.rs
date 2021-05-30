use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{fmt, io, path::Path, process::Command};

/// Contains utility functions useful for neovim/vim operations
pub mod utils;

/// Represents a vim variable to be extracted
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VimVar<Name: AsRef<str>> {
    cmd: Cmd,
    scope: Scope,
    name: Name,
}

impl<Name: AsRef<str>> VimVar<Name> {
    /// Creates a new vim variable definition that can be used later to
    /// load the variable's contents
    pub fn new(cmd: Cmd, scope: Scope, name: Name) -> Self {
        Self { cmd, scope, name }
    }

    /// Returns [`Cmd`] tied to variable
    pub fn cmd(&self) -> Cmd {
        self.cmd
    }

    /// Returns [`Scope`] tied to variable
    pub fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns name tied to variable
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl<Name: AsRef<str>> VimVar<Name> {
    /// Retrieves a vim variable with `b:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_buffer_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Buffer;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_buffer_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_buffer_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Buffer;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `w:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_window_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Window;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_window_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_window_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Window;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `t:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_tabpage_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Tabpage;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_tabpage_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_tabpage_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Tabpage;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `l:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_local_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Local;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_local_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_local_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Local;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `s:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_script_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Script;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_script_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_script_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Script;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `a:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_function_arg_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::FunctionArg;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_function_arg_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_function_arg_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::FunctionArg;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `g:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_global_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Global;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_global_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_global_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Global;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }

    /// Retrieves a vim variable with `v:` scope using whatever neovim/vim
    /// instance is available in the current path
    pub fn load_vim_var(name: Name, allow_zero: bool) -> io::Result<Option<Value>> {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Vim;
        Self { cmd, scope, name }.load(allow_zero)
    }

    /// Same as [`Self::load_vim_var`], but converts to the specified type
    /// after being loaded, returing an [`io::Error`] if failing to convert
    pub fn load_typed_vim_var<T>(name: Name, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let cmd = utils::find_cmd()?;
        let scope = Scope::Vim;
        Self { cmd, scope, name }.load_typed(allow_zero)
    }
}

impl<Name: AsRef<str>> VimVar<Name> {
    /// Loads variable with [`Self::load`] and then attempts to convert it
    /// to the specified type
    ///
    /// ### Notes
    ///
    /// * If `allow_zero` is true, then a value of 0 is considered the value of
    ///   the variable rather than vim's default of not being found
    pub fn load_typed<T>(&self, allow_zero: bool) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.load(allow_zero)?
            .map(|value| serde_json::from_value(value).map_err(Into::into))
            .transpose()
    }

    /// Loads the variable's value using neovim's headless mode or vim's ex
    /// mode using the default vimrc available in scope
    ///
    /// ### Notes
    ///
    /// * Will leverage [`utils::find_vimrc`] to load in the appropriate vimrc
    ///   during ex mode
    /// * If `allow_zero` is true, then a value of 0 is considered the value of
    ///   the variable rather than vim's default of not being found
    pub fn load(&self, allow_zero: bool) -> io::Result<Option<Value>> {
        let vimrc = utils::find_vimrc()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "vimrc not found"))?;
        self.load_with_config(vimrc, allow_zero)
    }

    /// Loads variable with [`Self::load_with_config`] and then attempts to
    /// convert it to the specified type
    ///
    /// ### Notes
    ///
    /// * If `allow_zero` is true, then a value of 0 is considered the value of
    ///   the variable rather than vim's default of not being found
    pub fn load_typed_with_config<P: AsRef<Path>, T>(
        &self,
        config: P,
        allow_zero: bool,
    ) -> io::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.load_with_config(config, allow_zero)?
            .map(|value| serde_json::from_value(value).map_err(Into::into))
            .transpose()
    }

    /// Loads the variable's value using neovim's headless mode or vim's ex
    /// mode
    ///
    /// ### Notes
    ///
    /// * Spawns a vim process whose goal is to print out the contents of a
    ///   variable as a JSON string
    /// * Leverages batch & ex modes with redir to execute and capture output
    /// * Relies on the variable being available upon loading vim configs
    /// * If `allow_zero` is true, then a value of 0 is considered the value of
    ///   the variable rather than vim's default of not being found
    pub fn load_with_config<P: AsRef<Path>>(
        &self,
        config: P,
        allow_zero: bool,
    ) -> io::Result<Option<Value>> {
        let cmd = self.cmd;
        let scope = self.scope.as_str();
        let var = self.name.as_ref();

        let full_cmd = {
            if config.as_ref().as_os_str().is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "path to vimrc is required for neovim/vim",
                ));
            }

            // NOTE: We have a lot of settings being applied, so documenting
            //       them here
            //
            //       1. -Es is our silent, batch, ex mode
            //       2. -i NONE removes shada/viminfo file reading and writing
            //       3. -u "{}" loads our vimrc, which is required as -Es does
            //          not load vim scripts by default
            //       4. +set nonumber is used to turn off line numbers, which
            //          are getting picked up by neovim/vim in vimrc configs
            //          and showing up in output
            //       5. redir writes to a register our message (json) and then
            //          places it in our buffer
            //       6. prints out the content in our buffer (current line)
            format!(
                r#"{} -Es -i NONE -u "{}" '+set nonumber' '+redir => m | echon json_encode(get({}, "{}")) | redir END | put=m' '+%p' '+qa!'"#,
                cmd,
                config.as_ref().to_string_lossy(),
                scope,
                var,
            )
        };

        // TODO: Support windows here (won't have sh)
        let output = Command::new("sh").arg("-c").arg(full_cmd).output()?;

        // If our program failed, we want to report the failure
        //
        // NOTE: neovim/vim seems to return exit code 1; so, for now we'll
        //       ignore that specific exit code for now
        if !output.status.success() && (output.status.code() != Some(1)) {
            let code = output
                .status
                .code()
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| String::from("--"));
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "[Exit code {}]: {}",
                    code,
                    String::from_utf8_lossy(&output.stderr).trim()
                )
                .as_str(),
            ));
        }

        let output_string = String::from_utf8_lossy(&output.stdout);

        // Report a better error than the serde one if the output was empty
        if output_string.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Result from {} was empty", self.cmd),
            ));
        }

        let value: Value = serde_json::from_str(output_string.trim()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse as JSON: \"{}\"", output_string.trim()),
            )
        })?;

        if !allow_zero && value == serde_json::json!(0) {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }
}

/// Represents type of vim instance being used
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cmd {
    Neovim,
    Vim,
}

impl Cmd {
    /// Converts to a str representing command
    ///
    /// ### Examples
    ///
    /// ```
    /// use vimvar::Cmd;
    ///
    /// assert_eq!(Cmd::Neovim.as_str(), "nvim");
    /// assert_eq!(Cmd::Vim.as_str(), "vim");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vim => "vim",
            Self::Neovim => "nvim",
        }
    }
}

impl fmt::Display for Cmd {
    /// Writes cmd using the `as_str()` representation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a vim variable scope
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scope {
    Nothing,
    Buffer,
    Window,
    Tabpage,
    Global,
    Local,
    Script,
    FunctionArg,
    Vim,
}

impl Default for Scope {
    /// Returns global as default
    fn default() -> Self {
        Self::Global
    }
}

impl Scope {
    /// Converts to a str representing scope
    ///
    /// ### Examples
    ///
    /// ```
    /// use vimvar::Scope;
    ///
    /// assert_eq!(Scope::Nothing.as_str(), "");
    /// assert_eq!(Scope::Buffer.as_str(), "b:");
    /// assert_eq!(Scope::Window.as_str(), "w:");
    /// assert_eq!(Scope::Tabpage.as_str(), "t:");
    /// assert_eq!(Scope::Global.as_str(), "g:");
    /// assert_eq!(Scope::Local.as_str(), "l:");
    /// assert_eq!(Scope::Script.as_str(), "s:");
    /// assert_eq!(Scope::FunctionArg.as_str(), "a:");
    /// assert_eq!(Scope::Vim.as_str(), "v:");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nothing => "",
            Self::Buffer => "b:",
            Self::Window => "w:",
            Self::Tabpage => "t:",
            Self::Global => "g:",
            Self::Local => "l:",
            Self::Script => "s:",
            Self::FunctionArg => "a:",
            Self::Vim => "v:",
        }
    }
}

impl fmt::Display for Scope {
    /// Writes scope using the `as_str()` representation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
