// SPDX-FileCopyrightText: Copyright © 2020-2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{arg, ArgMatches, Command};
use std::fs::File;
use stone::payload::MetaKind;
use thiserror::Error;

pub fn command() -> Command {
    Command::new("inspect")
        .about("Examine raw stone files")
        .long_about("Show detailed (debug) information on a local `.stone` file")
        .arg(arg!(<PATH> ... "files to inspect").value_parser(clap::value_parser!(PathBuf)))
}

///
/// Inspect the given .stone files and print results
///
pub fn handle(args: &ArgMatches) -> Result<(), Error> {
    let paths = args
        .get_many::<PathBuf>("PATH")
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    // Process each input path in order.
    for path in paths {
        let rdr = File::open(path).map_err(Error::IO)?;
        let reader = stone::read(rdr).map_err(Error::Format)?;
        // Grab the header version
        println!(
            "{path:?} = stone container version {:?}",
            reader.header.version()
        );

        for record in reader.metadata {
            match record.kind {
                MetaKind::String(s) => println!("Record: `{:?}` = {}", record.tag, s),
                MetaKind::Int64(i) => {
                    println!("Record: `{:?}` = {}", record.tag, i);
                }
                MetaKind::Uint64(i) => {
                    println!("Record: `{:?}` = {}", record.tag, i);
                }
                _ => println!("Record: {record:?}"),
            }
        }
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Read failure")]
    IO(#[from] std::io::Error),

    #[error("Format failure")]
    Format(#[from] stone::read::Error),
}
