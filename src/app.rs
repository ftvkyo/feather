use std::fs::OpenOptions;

use clap::{Parser, ValueEnum};

use crate::{file::produce_stl, view::View, Shape};

#[derive(ValueEnum, Clone, Debug)]
enum AppMode {
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
struct AppArgs {
    #[arg(short, long, value_enum, default_value_t)]
    mode: AppMode,
}

pub struct App {
    args: AppArgs,
    title: String,
}

impl App {
    pub fn new<S: ToString>(title: S) -> Self {
        Self {
            args: AppArgs::parse(),
            title: title.to_string(),
        }
    }

    pub fn run(self, shape: impl Shape) {
        match self.args.mode {
            AppMode::View => {
                let view = View::new(self.title);
                view.run(shape.mesh_render());
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

                let stl = produce_stl(shape.mesh());
                stl_io::write_stl(&mut file, stl.iter()).unwrap();
            }
        }
    }
}
