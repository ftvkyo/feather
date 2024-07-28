use std::fs::OpenOptions;

use clap::{Parser, ValueEnum};
use stl_io::IndexedMesh;

use crate::{produce_stl, view::View};


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

    pub fn run(self, mesh: IndexedMesh) {
        match self.args.mode {
            AppMode::View => {
                let view = View::new(self.title);
                view.run(mesh);
            },
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

                stl_io::write_stl(&mut file, produce_stl(mesh).iter()).unwrap();
            },
        }

    }
}
