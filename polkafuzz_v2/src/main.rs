extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};
use std::process::Command;
use structopt::StructOpt;
use Cli::*;
use std::env;
use std::path::PathBuf;

pub fn root_dir() -> Result<PathBuf, Error> {
    let p = env::var("CARGO_MANIFEST_DIR")
        .map(From::from)
        .or_else(|_| env::current_dir())?;
    Ok(p.parent().unwrap().to_path_buf())
}

pub fn corpora_dir() -> Result<PathBuf, Error> {
    let p = root_dir()?.join("corpora");
    Ok(p)
}

#[derive(StructOpt, Debug)]
enum Cli {
    /// Run fuzzer
    #[structopt(name = "fuzz")]
    Fuzz {
        // Which client to rrun
        #[structopt(
            possible_values = &self::Clients::variants(),
            case_insensitive = true
        )]
        client: Clients,
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
}

arg_enum! {
    #[derive(Copy, Clone, StructOpt, Debug)]
    enum Targets {
        ChainSpec,
        MultiaddrFromStr,
        MultiaddrTryFrom,
        MultihashFromBytes,
        DecodeBabePreDigest,
        PublicKeyFromProtobufEncoding,
        PeerIdFromBytes,
        DecodeBabeNextEpoch,
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
        Fuzz { client, engine, target } => {
            fuzz_target(client, engine, target)?;
        }
        // list all targets
        ListTargets => {
            list_targets()?;
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

fn rust_fuzzers(engine: Engines, target_name: &str, client_name: &str) -> Result<(),Error> {
    match engine {
        Engines::LibFuzzer => {
            let res = Command::new("cargo")
                .arg("fuzz")
                .arg("run")
                .arg(target_name.to_owned() + "_libfuzzer")        
                .arg(corpora_dir()?.join(target_name))
                .current_dir("fuzzers/".to_owned() + client_name)
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
                .arg(corpora_dir()?.join(target_name))
                .arg("--")
                .arg("--cores")
                .arg("1")
                .current_dir("fuzzers/".to_owned() + client_name)
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

fn fuzz_target(client: Clients, engine: Engines, target: Targets) -> Result<(), Error> {
    let target_name = match target {
        Targets::ChainSpec => "chain_spec",
        Targets::MultiaddrFromStr => "multiaddr_from_str",
        Targets::MultiaddrTryFrom => "multiaddr_try_from",
        Targets::MultihashFromBytes => "multihash_from_bytes",
        Targets::DecodeBabePreDigest => "decode_babepredigest",
        Targets::PublicKeyFromProtobufEncoding => "publickey_from_protobuf_encoding",
        Targets::PeerIdFromBytes => "peerid_from_bytes",
        Targets::DecodeBabeNextEpoch => "decode_babenextepoch",
    };
    let client_name = match client {
        Clients::Gossamer => "gossamer",
        Clients::Substrate => "substrate",
        Clients::Smoldot => "smoldot",
    };
    match client {
      Clients::Gossamer => {
        //go_fuzzers(engine, target_name);
      },
      Clients::Substrate => {
        rust_fuzzers(engine, target_name, client_name).unwrap()
      },
      Clients::Smoldot => {
        rust_fuzzers(engine, target_name, client_name).unwrap()
      },
    };
    Ok(())
}
