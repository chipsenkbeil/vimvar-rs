/// Contains utility functions useful for neovim/vim operations
pub mod utils;

mod var;
pub use var::*;

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::io;

/// Retrieves a vim variable with `b:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_buffer_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Buffer;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_buffer_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_buffer_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Buffer;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `w:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_window_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Window;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_window_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_window_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Window;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `t:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_tabpage_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Tabpage;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_tabpage_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_tabpage_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Tabpage;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `l:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_local_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Local;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_local_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_local_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Local;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `s:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_script_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Script;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_script_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_script_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Script;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `a:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_function_arg_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::FunctionArg;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_function_arg_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_function_arg_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::FunctionArg;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `g:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_global_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Global;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_global_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_global_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Global;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}

/// Retrieves a vim variable with `v:` scope using whatever neovim/vim
/// instance is available in the current path
pub fn load_vim_var(name: &str, allow_zero: bool) -> io::Result<Option<Value>> {
    let cmd = utils::find_cmd()?;
    let scope = Scope::Vim;
    VimVar::new(cmd, scope, name).load(allow_zero)
}

/// Same as [`Self::load_vim_var`], but converts to the specified type
/// after being loaded, returing an [`io::Error`] if failing to convert
pub fn load_typed_vim_var<T>(name: &str, allow_zero: bool) -> io::Result<Option<T>>
where
    T: DeserializeOwned,
{
    let cmd = utils::find_cmd()?;
    let scope = Scope::Vim;
    VimVar::new(cmd, scope, name).load_typed(allow_zero)
}
