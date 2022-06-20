// Copyright 2022 Oxide Computer Company

//! Use `rustfmt` to format generated code:
//! ```
//! let codegen = quote::quote!{ struct Foo { bar: String } };
//! let formatted: String = rustfmt_wrapper::rustfmt(codegen).unwrap();
//! ```

use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use thiserror::Error;

pub mod config;

#[derive(Error, Debug)]
pub enum Error {
    /// Command `rustfmt` could not be found
    #[error("rustfmt is not installed")]
    NoRustfmt,
    /// Command `rustfmt` produced an error at runtime.
    #[error("rustfmt runtime error")]
    Rustfmt(String),
    /// Error with file IO
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// Error from reading stdin of rustfmt
    #[error(transparent)]
    Conversion(#[from] std::string::FromUtf8Error),
}

/// Use the `rustfmt` command to format the input.
///
/// The only non-default `rustfmt` configuration is `wrap_comments = true`.
pub fn rustfmt<T: ToString>(input: T) -> Result<String, Error> {
    let config = config::Config {
        edition: Some(config::Edition::Edition2018),
        wrap_comments: Some(true),
        ..Default::default()
    };
    rustfmt_config(config, input)
}

/// Use the `rustfmt` command to format the input with the given [`Config`].
///
/// [`Config`]: config::Config
pub fn rustfmt_config<T: ToString>(mut config: config::Config, input: T) -> Result<String, Error> {
    let input = input.to_string();

    // rustfmt's default edition is 2015; our default is 2021.
    if config.edition.is_none() {
        config.edition = Some(config::Edition::Edition2018);
    }

    let mut builder = tempfile::Builder::new();
    builder.prefix("rustfmt-wrapper");
    let outdir = builder.tempdir().expect("failed to create tmp file");

    let rustfmt_config_path = outdir.as_ref().join("rustfmt.toml");
    std::fs::write(
        rustfmt_config_path,
        toml::to_string_pretty(&config).unwrap(),
    )?;

    let rustfmt = which_rustfmt().ok_or(Error::NoRustfmt)?;

    let mut command = Command::new(&rustfmt)
        .arg(format!("--config-path={}", outdir.path().to_str().unwrap()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = command.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = command.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(Error::Rustfmt(String::from_utf8(output.stderr)?))
    }
}

fn which_rustfmt() -> Option<PathBuf> {
    match env::var_os("RUSTFMT") {
        Some(which) => {
            if which.is_empty() {
                None
            } else {
                Some(PathBuf::from(which))
            }
        }
        None => toolchain_find::find_installed_component("rustfmt"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, rustfmt, rustfmt_config};
    use newline_converter::dos2unix;
    use quote::quote;

    #[test]
    fn test_basics() {
        let code = quote! { struct Foo { bar: String } };
        assert_eq!(
            dos2unix(rustfmt(code).unwrap().as_str()),
            "struct Foo {\n    bar: String,\n}\n"
        );
    }

    #[test]
    fn test_doc_comments() {
        let comment = "This is a very long doc comment that could span \
        multiple lines of text. For the purposes of this test, we're hoping \
        that it gets formatted into a single, nice doc comment.";
        let code = quote! {
           #[doc = #comment]
           struct Foo { bar: String }
        };

        let config = Config {
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        };

        assert_eq!(
            dos2unix(rustfmt_config(config, code).unwrap().as_str()),
            r#"///This is a very long doc comment that could span multiple lines of text. For
/// the purposes of this test, we're hoping that it gets formatted into a
/// single, nice doc comment.
struct Foo {
    bar: String,
}
"#,
        );
    }

    #[test]
    fn test_narrow_call() {
        let code = quote! {
            async fn go() {
                let _ = Client::new().operation_id().send().await?;
            }
        };

        let config = Config {
            max_width: Some(45),
            ..Default::default()
        };

        assert_eq!(
            dos2unix(rustfmt_config(config, code).unwrap().as_str()),
            "async fn go() {
    let _ = Client::new()
        .operation_id()
        .send()
        .await?;
}\n"
        );
    }
}
