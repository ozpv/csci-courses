use super::compiler::{Compiler, CompilerError, SourceFile};
use axum::response::{Html, IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::env::temp_dir;

#[derive(Deserialize, Serialize)]
pub struct CompileOptions {
    pub source_code: Vec<SourceFile>,
}

pub async fn compile(
    Json(options): Json<CompileOptions>,
) -> Result<impl IntoResponse, CompilerError> {
    let compiler = Compiler::new(options.source_code, temp_dir(), "build-gcc").await?;

    let res = compiler.run().await?;

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
            Ok(Html(format!("<a>{err}</a>")))
        }
        // success
        Some(0) => {
            println!("Success: {out}");
            Ok(Html(format!("<a>Success: {out}</a>")))
        }
        // else
        // TODO: allow other status codes
        Some(_) => {
            println!("Invalid: {out}");
            println!("Invalid: {err}");
            Ok(Html(format!("<a>invalid status code: {err}</a>")))
        }
        None => Ok(Html("<a>No status codes found</a>".to_string())),
    }
}
