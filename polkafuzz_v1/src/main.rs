extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::process::Command;
use structopt::StructOpt;
use Cli::*;

#[derive(StructOpt, Debug)]
enum Cli {
    /// Run fuzzer
    #[structopt(name = "fuzz")]
    Fuzz {
        // Which engine to run
        #[structopt(
            possible_values = &self::Engines::variants(),
            case_insensitive = true
        )]
        engine: Engines,
        /// Which target to run
        #[structopt(
            possible_values = &self::Targets::variants(),
            case_insensitive = true
        )]
        target: Targets,
    },
    /// List all available fuzzing targets
    #[structopt(name = "list")]
    ListTargets,
    /// Test a target with an empty input or hardcoded inputs
    #[structopt(name = "test")]
    Tests {
        #[structopt(
            possible_values = &self::Targets::variants(),
            case_insensitive = true
        )]
        target: Targets,
    },
    /// Reproduce crash with crash file or hardcoded inputs
    #[structopt(name = "reproduce")]
    Reproduce {
        #[structopt(
            possible_values = &self::Clients::variants(),
            case_insensitive = true
        )]
        client: Clients,
        #[structopt(
            possible_values = &self::Targets::variants(),
            case_insensitive = true
        )]
        target: Targets,
        crash_file: String,
    },
}

arg_enum! {
    #[derive(Copy, Clone, StructOpt, Debug)]
    enum Targets {
        ChainSpec,
        MultiaddrFromStr,
        MultiaddrTryFrom,
        DecodeBabePreDigest,
    }
}

arg_enum! {
    #[derive(Copy, Clone, StructOpt, Debug)]
    enum Engines {
        LibFuzzer,
        LibAFL,
    }
}

arg_enum! {
    #[derive(Copy, Clone, StructOpt, Debug)]
    enum Clients {
        Gossamer,
        Smoldot,
        Substrate,
    }
}

/// Parsing of CLI arguments
fn run() -> Result<(), Error> {
    let cli = Cli::from_args();

    match cli {
        // Fuzz targets
        Fuzz { engine, target } => {
            fuzz_target(engine, target)?;
        }
        // list all targets
        ListTargets => {
            list_targets()?;
        }
        // Test a target with empty input or hardcoded input
        Tests { target } => {
            test(target)?;
        }
        // Reproduce crash with crash file or hardcoded inputs
        Reproduce {
            client,
            target,
            crash_file,
        } => {
            reproduce_crash(client, target, crash_file)?;
        }
    }
    Ok(())
}

/// Main function catching errors
fn main() {
    if let Err(e) = run() {
        eprintln!("[-] {}", e);
        for cause in e.iter_chain().skip(1) {
            eprintln!("[-] caused by: {}", cause);
        }
        ::std::process::exit(1);
    }
}

/// List all targets available
fn list_targets() -> Result<(), Error> {
    println!("[+] List of available targets:");
    for target in Targets::variants().iter() {
        println!("    {}", target);
    }
    Ok(())
}

fn test(target: Targets) -> Result<(), Error> {
    let test_input = [0; 65];
    match target {
        Targets::ChainSpec => {
            smoldot::smoldot_chain_spec_from_json_bytes(&test_input);
            substrate::substrate_chain_spec_from_json_bytes(&test_input);
            gossamer::gossamer_genesis_json_from_bytes(&test_input);
        }
        Targets::MultiaddrFromStr => {
            smoldot::smoldot_multiaddr_from_str(&test_input);
            substrate::substrate_multiaddr_from_str(&test_input);
            gossamer::gossamer_new_multiaddr(&test_input);
        }
        Targets::MultiaddrTryFrom => {
            smoldot::smoldot_multiaddr_try_from(&test_input);
            substrate::substrate_multiaddr_try_from(&test_input);
            gossamer::gossamer_new_multiaddr_bytes(&test_input);
        }
        Targets::DecodeBabePreDigest => {
            smoldot::smoldot_decode_babepredigest(&test_input);
            substrate::substrate_decode_babepredigest(&test_input);
            gossamer::gossamer_decode_babepredigest(&test_input);
        }
    }
    Ok(())
}

fn fuzz_target(engine: Engines, target: Targets) -> Result<(), Error> {
    let target_name = match target {
        Targets::ChainSpec => "chain_spec",
        Targets::MultiaddrFromStr => "multiaddr_from_str",
        Targets::MultiaddrTryFrom => "multiaddr_try_from",
        Targets::DecodeBabePreDigest => "decode_babepredigest",
    };
    match engine {
        Engines::LibFuzzer => {
            let res = Command::new("cargo")
                .arg("fuzz")
                .arg("run")
                .arg(target_name.to_owned() + "_libfuzzer")
                .spawn()
                .context(format!("cargo command failed to start"))?
                .wait()
                .context(format!("cargo command failed to wait"));
            if !res.as_ref().unwrap().success() {
                println!("{}", res.unwrap());
                ::std::process::exit(1);
            }
        }
        Engines::LibAFL => {
            let res = Command::new("cargo")
                .arg("libafl")
                .arg("run")
                .arg(target_name.to_owned() + "_libafl")
                .arg("--")
                .arg("--cores")
                .arg("1")
                .spawn()
                .context(format!("cargo command failed to start"))?
                .wait()
                .context(format!("cargo command failed to wait"));
            if !res.as_ref().unwrap().success() {
                println!("{}", res.unwrap());
                ::std::process::exit(1);
            }
        }
    };
    Ok(())
}

fn reproduce_crash(_client: Clients, _target: Targets, _crash_file: String) -> Result<(), Error> {
    Ok(())
}
