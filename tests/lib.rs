use serde_json::json;
use tempfile::{NamedTempFile, TempPath};
use vimvar::*;

static TEST_VIMRC: &str = r#"
let b:my_buffer_var = 'some buffer value'
let w:my_window_var = 'some window value'
let t:my_tabpage_var = 'some tabpage value'
let g:my_global_var = 'some global value'
"#;

fn make_test_vimrc() -> TempPath {
    use std::io::Write;
    let mut file = NamedTempFile::new().unwrap();
    file.as_file_mut().write_all(TEST_VIMRC.as_bytes()).unwrap();
    file.into_temp_path()
}

macro_rules! impl_tests {
    ($cmd:expr) => {
        #[test]
        fn can_load_buffer_variable() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Buffer, "my_buffer_var");
            let value = var
                .load_with_config(path, false)
                .expect("Failed to load variable");

            assert_eq!(value, Some(json!("some buffer value")));
        }

        #[test]
        fn can_load_window_variable() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Window, "my_window_var");
            let value = var
                .load_with_config(path, false)
                .expect("Failed to load variable");

            assert_eq!(value, Some(json!("some window value")));
        }

        #[test]
        fn can_load_tabpage_variable() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Tabpage, "my_tabpage_var");
            let value = var
                .load_with_config(path, false)
                .expect("Failed to load variable");

            assert_eq!(value, Some(json!("some tabpage value")));
        }

        #[test]
        fn can_load_global_variable() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Global, "my_global_var");
            let value = var
                .load_with_config(path, false)
                .expect("Failed to load variable");

            assert_eq!(value, Some(json!("some global value")));
        }

        #[test]
        fn can_load_variable_as_specific_type() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Global, "my_global_var");
            let value: Option<String> = var
                .load_typed_with_config(path, false)
                .expect("Failed to load variable");

            assert_eq!(value.as_deref(), Some("some global value"));
        }

        #[test]
        fn reports_error_when_loading_variable_as_wrong_specific_type() {
            let path = make_test_vimrc();

            let var = VimVar::new($cmd, Scope::Global, "my_global_var");
            let result = var.load_typed_with_config::<_, usize>(path, false);

            assert_eq!(result.is_err(), true);
        }
    };
}

mod nvim {
    use super::*;
    impl_tests!(Cmd::Neovim);
}

mod vim {
    use super::*;
    impl_tests!(Cmd::Vim);
}
