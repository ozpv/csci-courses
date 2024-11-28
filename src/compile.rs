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
#[derive(Deserialize)]
pub enum AssignmentType {
    Example,
}

#[derive(Deserialize)]
pub struct CompileJson {
    cpp_code: String,
    // json "null" for None
    // and just the enum's name for
    // the assignment example
    // {"assignment":"Example"}
    assignment: Option<AssignmentType>,
}

pub async fn compile(Json(options): Json<CompileJson>) -> impl IntoResponse {
    let Ok(container) = Container::new(
        // run docker build --tag 'build-gcc' .
        // find image in the example dir
        // this is the docker image name
        "build-gcc",
        // source code
        &options.cpp_code,
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
        None => Html(format!("<a>No status codes found</a>")),
    }
}
