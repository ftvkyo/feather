use std::{fs::OpenOptions, path::PathBuf};

use clap::{Parser, ValueEnum};

use crate::{geometry::{primitives::Triangles, Geometry3D}, render::view::View};

#[derive(ValueEnum, Clone, Debug)]
pub enum AppMode {
    View,
    Output,
}

impl Default for AppMode {
    fn default() -> Self {
        Self::View
    }
}

#[derive(Parser, Debug)]
#[command()]
pub struct AppArgs {
    #[arg(short, long, value_enum, default_value_t)]
    pub mode: AppMode,
    pub file: PathBuf,
}

pub struct App {
    pub args: AppArgs,
    title: String,
}

impl App {
    pub fn new<S: ToString>(title: S) -> Self {
        let args = AppArgs::parse();

        Self {
            title: format!("{} - {:?}", title.to_string(), args.file),
            args,
        }
    }

    pub fn run(&self, geometry: Geometry3D) {
        match self.args.mode {
            AppMode::View => {
                let view = View::new(&self.title);
                view.run(geometry);
            }
            AppMode::Output => {
                let output_dir = "out/";
                let output_path = format!("{}/{}.stl", output_dir, self.title);

                std::fs::create_dir_all(output_dir).unwrap();

                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(output_path)
                    .unwrap();

                Triangles::new(geometry.iter_triangles().collect()).stl(&mut file).unwrap();
            }
        }
    }
}
