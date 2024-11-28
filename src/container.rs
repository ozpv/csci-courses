use std::path::PathBuf;
use std::process::Output;
use tokio::fs::{create_dir_all, remove_dir_all, write};
use tokio::process::Command;
use uuid::Uuid;

use crate::compile::AssignmentType;

const EXAMPLE_READ: &str = "1
2
3
4
Works
";

pub struct Container<'a> {
    /// C++ source code
    /// sent from textarea
    /// for some reason, rust thinks it isnt used but is
    /// in the new function
    #[allow(dead_code)]
    cpp_code: &'a str,
    /// Output directory of code and extra files
    output_dir: PathBuf,
    // id
    id: Uuid,
    // name of the docker image, expects you already
    // have ran docker build --tag '<name-here>' .
    // to build the Dockerfile in the pwd
    image_name: &'a str,
    /// info in compile.rs
    #[allow(dead_code)]
    assignment_type: Option<AssignmentType>,
}

impl<'a> Container<'a> {
    pub async fn new(
        image_name: &'a str,
        cpp_code: &'a str,
        output_dir: PathBuf,
        assignment_type: Option<AssignmentType>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();

        let output_dir = output_dir.join("csci-courses").join(id.to_string());

        create_dir_all(&output_dir).await?;

        write(output_dir.join("main.cpp"), cpp_code).await?;

        // copy the extra shit if the student needs it
        match assignment_type {
            Some(AssignmentType::Example) => {
                // here, we would really copy the file needed to the dir
                // and not write from a &'static str
                write(output_dir.join("read.txt"), EXAMPLE_READ).await?;
            }
            None => (),
        };

        println!("{}", output_dir.display());

        Ok(Self {
            cpp_code,
            output_dir,
            id,
            image_name,
            assignment_type,
        })
    }

    pub async fn run(&self) -> Result<Output, Box<dyn std::error::Error>> {
        Ok(Command::new("docker")
            .arg("run")
            // set name of container to id
            .arg("--name")
            .arg(self.id.to_string())
            // -v: mount our temp dir to
            // WORKDIR in the container
            // this is important so the container can
            // access the main.cpp or read.txt files
            .arg("-v")
            // make sure to change csci-courses to WORKDIR
            // on update
            .arg(format!(
                "{}:/csci-courses/code/:z",
                self.output_dir.display()
            ))
            // --pull never: don't search for image
            // if it doesn't exist
            .arg("--pull")
            .arg("never")
            // --quiet: don't output pull
            .arg("--quiet")
            // --rm: don't persist the container after execution
            .arg("--rm")
            // our image name
            .arg(self.image_name)
            .output()
            .await?)
    }
}

/// When no longer in scope, clean up the files
impl<'a> Drop for Container<'a> {
    fn drop(&mut self) {
        let dir = self.output_dir.clone();

        tokio::spawn(async move { remove_dir_all(dir).await });
    }
}
