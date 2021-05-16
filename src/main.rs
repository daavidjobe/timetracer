mod config;
use anyhow::{anyhow, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Timetracer", about = "Keep track of working hours.")]
pub enum Opt {
    New {
        #[structopt(long, short)]
        project: String,
    },
    Start {
        #[structopt(long, short)]
        project: String,
    },
    Stop {
        #[structopt(long, short)]
        project: String,
    },
    Print {
        #[structopt(long, short)]
        project: Option<String>,
        #[structopt(long, short, default_value = "10")]
        limit: usize,
    },
    Current,
}

pub fn run(opt: Opt) -> Result<()> {
    match opt {
        Opt::New { project: String } => Ok(()),
        _ => {
            unimplemented!()
        }
    }
}

fn main() {
    let config = crate::config::read();
    println!("Cfg: {:?}", config);
    match run(Opt::from_args()) {
        Ok(()) => println!("Done."),
        Err(e) => eprintln!("Error: {}", anyhow!("{}", e)),
    }
}
