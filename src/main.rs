// Copyright 2018 Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! The main CLI driver logic.

extern crate chrono;
extern crate failure;
extern crate itertools;
extern crate slurm;
#[macro_use] extern crate structopt;
extern crate users;

use failure::Error;
use std::process;
use structopt::StructOpt;


mod recent;
mod status;


#[derive(Debug, StructOpt)]
#[structopt(name = "slurmplus", about = "Better commands for interacting with Slurm.")]
enum SlurmPlusCli {
    #[structopt(name = "recent")]
    /// Summarize recently-run jobs
    Recent(recent::RecentCommand),

    #[structopt(name = "status")]
    /// Get the status of a job
    Status(status::StatusCommand),
}

impl SlurmPlusCli {
    fn cli(self) -> Result<i32, Error> {
        match self {
            SlurmPlusCli::Recent(cmd) => cmd.cli(),
            SlurmPlusCli::Status(cmd) => cmd.cli(),
        }
    }
}


fn main() {
    let program = SlurmPlusCli::from_args();

    process::exit(match program.cli() {
        Ok(code) => code,

        Err(e) => {
            eprintln!("slurmplus: the command failed");
            for cause in e.causes() {
                eprintln!("  caused by: {}", cause);
            }
            1
        },
    });
}
