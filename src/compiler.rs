use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Output;
use thiserror::Error;
use tokio::{
    fs::{create_dir_all, remove_dir_all, write},
    process::Command,
};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Failed to create directory for compiler output")]
    DirectoryCreation,
    #[error("Source files are empty")]
    EmptySourceFiles,
    #[error("Failed to write files to output_dir")]
    Write,
    #[error("Failed to execute docker container")]
    SpawnError,
}

impl IntoResponse for CompilerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            CompilerError::EmptySourceFiles => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self.to_string()).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct FileData {
    name: String,
    contents: String,
}

#[allow(unused)]
impl FileData {
    pub fn new(name: String, contents: String) -> Self {
        Self { name, contents }
    }

    pub fn add_extension(mut self, ext: &str) -> Self {
        self.name.push_str(ext);
        self
    }
}

#[derive(Deserialize, Serialize)]
pub enum SourceFile {
    C(FileData),
    Cpp(FileData),
    CHeader(FileData),
    CppHeader(FileData),
}

impl SourceFile {
    fn extension(&self) -> &'static str {
        match self {
            SourceFile::C(_) => ".c",
            SourceFile::Cpp(_) => ".cpp",
            SourceFile::CHeader(_) => ".h",
            SourceFile::CppHeader(_) => ".hpp",
        }
    }

    fn unwrap(self) -> FileData {
        match self {
            SourceFile::C(x)
            | SourceFile::Cpp(x)
            | SourceFile::CHeader(x)
            | SourceFile::CppHeader(x) => x,
        }
    }
}

pub struct Compiler {
    output_dir: PathBuf,
    image_name: &'static str,
    id: Uuid,
    cleanup: bool,
}

impl Compiler {
    pub async fn new(
        source_code: impl IntoIterator<Item = SourceFile>,
        output_dir: PathBuf,
        image_name: &'static str,
    ) -> Result<Self, CompilerError> {
        let id = Uuid::new_v4();

        let output_dir = output_dir.join("output").join(id.to_string());

        create_dir_all(&output_dir)
            .await
            .map_err(|_| CompilerError::DirectoryCreation)?;

        for file in source_code.into_iter() {
            let ext = file.extension();
            let FileData { name, contents } = file.unwrap();
            let path = output_dir.join(format!("{name}{ext}"));
            write(path, contents)
                .await
                .map_err(|_| CompilerError::Write)?;
        }

        Ok(Self {
            output_dir,
            id,
            image_name,
            cleanup: true,
        })
    }

    pub async fn extras(
        &self,
        files: impl IntoIterator<Item = FileData>,
    ) -> Result<(), CompilerError> {
        for file in files.into_iter() {
            let FileData { name, contents } = file;
            write(name, contents)
                .await
                .map_err(|_| CompilerError::Write)?;
        }
        Ok(())
    }

    pub async fn cleanup(mut self, option: bool) -> Self {
        self.cleanup = option;
        self
    }

    pub async fn run(self) -> Result<Output, CompilerError> {
        let mut stdout = Command::new("docker");

        stdout
            .arg("run")
            .arg("--name")
            .arg(self.id.to_string())
            .arg("-v")
            .arg(format!("{}:/output/code/:z", self.output_dir.display()))
            .arg("--pull")
            .arg("never")
            .arg("--quiet");

        if self.cleanup {
            stdout.arg("--rm");
        }

        // execute
        stdout
            .arg(self.image_name)
            .output()
            .await
            .or_else(|_| Err(CompilerError::SpawnError))
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        if self.cleanup {
            let binding = self.output_dir.clone();
            tokio::spawn(async move { remove_dir_all(binding).await });
        }
    }
}
