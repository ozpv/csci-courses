use super::container::Container;
use axum::{
    response::{Html, IntoResponse},
    Json,
};
use serde::Deserialize;
use std::env::temp_dir;

/// This is for when say you
/// have a extra file the student needs
/// to read from
/// the container will copy it on execute
/// sadly needs to be hardcoded
/// but docker makes it easy to update
/// also could use `include_bytes!` instead
/// of writing a str into the source file
#[derive(Deserialize)]
pub enum AssignmentType {
    Example,
    // could expand on it like this
    // would have to add lifetime parameter
    // Example { filename: &'a str,  },
}

#[derive(Deserialize)]
pub enum SourceFile {
    // .cpp
    Cpp(String),
    // .hpp
    CppHeader(String),
    // .h
    CHeader(String),
    // .php
    Php(String),
}

impl SourceFile {
    /// Extension with leading dot
    pub fn full_extension<'a>(&self) -> &'a str {
        use SourceFile::*;
        match self {
            Cpp(_) => ".cpp",
            CppHeader(_) => ".hpp",
            CHeader(_) => ".h",
            Php(_) => ".php",
        }
    }

    /// Extracts the code from the wrapped enum
    pub fn unwrap_ref(&self) -> &String {
        use SourceFile::*;
        match self {
            Cpp(x) | CppHeader(x) | CHeader(x) | Php(x) => x,
        }
    }
}

#[derive(Deserialize)]
pub struct Options {
    pub source_code: Vec<SourceFile>,
    // json "null" for None
    // and just the enum's name for
    // the assignment
    // {"assignment":"Example"}
    pub assignment: Option<AssignmentType>,
}

pub async fn compile(Json(options): Json<Options>) -> impl IntoResponse {
    let Ok(container) = Container::new(
        // run docker build --tag 'build-gcc' .
        // find image in the example dir
        // this is the docker image name
        "build-gcc",
        // source code
        &options.source_code,
        // /tmp on unix
        temp_dir(),
        // the "assignment type"
        options.assignment,
    )
    .await
    else {
        return Html("<a>there was an error building the container</a>".to_string());
    };

    let Ok(res) = container.run().await else {
        return Html("<a>There was an error running the container</a>".to_string());
    };

    let err = String::from_utf8(res.stderr)
        .unwrap_or_else(|_| "stderr contained invalid UTF-8".to_string());

    let out = String::from_utf8(res.stdout)
        .unwrap_or_else(|_| "stdout contained invalid UTF-8".to_string());

    match res.status.code() {
        // there was an error
        // TODO: separate user errors from compiler errors
        Some(1) => {
            println!("Error: {err}");
            println!("Output: {out}");
            Html(format!("<a>{err}</a>"))
        }
        // success
        Some(0) => {
            println!("Success: {out}");
            Html(format!("<a>Success: {out}</a>"))
        }
        // else
        // TODO: allow other status codes
        Some(_) => {
            println!("Invalid: {out}");
            println!("Invalid: {err}");
            Html(format!("<a>invalid status code: {err}</a>"))
        }
        None => Html("<a>No status codes found</a>".to_string()),
    }
}
