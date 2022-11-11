extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use failure::Error;
use structopt::StructOpt;
use Cli::*;

#[derive(StructOpt, Debug)]
enum Cli {
    /// Run target function over multiple Polkadot client implementation
    #[structopt(name = "run")]
    Run {
        // Which client to run
        #[structopt(
            possible_values = &self::Clients::variants(),
            case_insensitive = true
        )]
        client: Clients,
        /// Which target to run
        #[structopt(
            possible_values = &self::Targets::variants(),
            case_insensitive = true
        )]
        target: Targets,
        /// Crash or sample file
        file_name: String,
    },
    /// List all available targets
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
        DecodeHeader,
    }
}

arg_enum! {
    #[derive(Copy, Clone, StructOpt, Debug)]
    enum Clients {
        Gossamer,
        Smoldot,
        Substrate,
        All,
    }
}

/// Parsing of CLI arguments
fn run() -> Result<(), Error> {
    let cli = Cli::from_args();
    match cli {
        // Run targets
        Run {
            client,
            target,
            file_name,
        } => {
            run_target(client, target, file_name)?;
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

fn run_gossamer(target: Targets, file_name: String) {
    match target {
        Targets::ChainSpec => {
            gossamer_lib::gossamer_chain_spec_from_json_bytes(&file_name);
        }
        Targets::MultiaddrFromStr => {
            gossamer_lib::gossamer_multiaddr_from_str(&file_name);
        }
        Targets::MultiaddrTryFrom => {
            gossamer_lib::gossamer_multiaddr_try_from(&file_name);
        }
        Targets::MultihashFromBytes => {
            gossamer_lib::gossamer_multihash_from_bytes(&file_name);
        }
        Targets::DecodeBabePreDigest => {
            gossamer_lib::gossamer_decode_babepredigest(&file_name);
        }
        Targets::PublicKeyFromProtobufEncoding => {
            gossamer_lib::gossamer_publickey_from_protobuf_encoding(&file_name);
        }
        Targets::PeerIdFromBytes => {
            gossamer_lib::gossamer_peerid_from_bytes(&file_name);
        }
        Targets::DecodeBabeNextEpoch => {
            gossamer_lib::gossamer_decode_babenextepoch(&file_name);
        }
        Targets::DecodeHeader => {
            gossamer_lib::gossamer_decode_header(&file_name);
        }
    };
}

fn run_substrate(target: Targets, file_name: String) {
    match target {
        Targets::ChainSpec => {
            substrate_lib::substrate_chain_spec_from_json_bytes(&file_name);
        }
        Targets::MultiaddrFromStr => {
            substrate_lib::substrate_multiaddr_from_str(&file_name);
        }
        Targets::MultiaddrTryFrom => {
            substrate_lib::substrate_multiaddr_try_from(&file_name);
        }
        Targets::MultihashFromBytes => {
            substrate_lib::substrate_multihash_from_bytes(&file_name);
        }
        Targets::DecodeBabePreDigest => {
            substrate_lib::substrate_decode_babepredigest(&file_name);
        }
        Targets::PublicKeyFromProtobufEncoding => {
            substrate_lib::substrate_publickey_from_protobuf_encoding(&file_name);
        }
        Targets::PeerIdFromBytes => {
            substrate_lib::substrate_peerid_from_bytes(&file_name);
        }
        Targets::DecodeBabeNextEpoch => {
            substrate_lib::substrate_decode_babenextepoch(&file_name);
        }
        Targets::DecodeHeader => {
            substrate_lib::substrate_decode_header(&file_name);
        }
    };
}

fn run_smoldot(target: Targets, file_name: String) {
    match target {
        Targets::ChainSpec => {
            smoldot_lib::smoldot_chain_spec_from_json_bytes(&file_name);
        }
        Targets::MultiaddrFromStr => {
            smoldot_lib::smoldot_multiaddr_from_str(&file_name);
        }
        Targets::MultiaddrTryFrom => {
            smoldot_lib::smoldot_multiaddr_try_from(&file_name);
        }
        Targets::MultihashFromBytes => {
            smoldot_lib::smoldot_multihash_from_bytes(&file_name);
        }
        Targets::DecodeBabePreDigest => {
            smoldot_lib::smoldot_decode_babepredigest(&file_name);
        }
        Targets::PublicKeyFromProtobufEncoding => {
            smoldot_lib::smoldot_publickey_from_protobuf_encoding(&file_name);
        }
        Targets::PeerIdFromBytes => {
            smoldot_lib::smoldot_peerid_from_bytes(&file_name);
        }
        Targets::DecodeBabeNextEpoch => {
            smoldot_lib::smoldot_decode_babenextepoch(&file_name);
        }
        Targets::DecodeHeader => {
            smoldot_lib::smoldot_decode_header(&file_name);
        }
    };
}

fn run_all(target: Targets, file_name: String) {
    match target {
        Targets::ChainSpec => {
            smoldot_lib::smoldot_chain_spec_from_json_bytes(&file_name);
            substrate_lib::substrate_chain_spec_from_json_bytes(&file_name);
            gossamer_lib::gossamer_chain_spec_from_json_bytes(&file_name);
        }
        Targets::MultiaddrFromStr => {
            smoldot_lib::smoldot_multiaddr_from_str(&file_name);
            substrate_lib::substrate_multiaddr_from_str(&file_name);
            gossamer_lib::gossamer_multiaddr_from_str(&file_name);
        }
        Targets::MultiaddrTryFrom => {
            smoldot_lib::smoldot_multiaddr_try_from(&file_name);
            substrate_lib::substrate_multiaddr_try_from(&file_name);
            gossamer_lib::gossamer_multiaddr_try_from(&file_name);
        }
        Targets::MultihashFromBytes => {
            smoldot_lib::smoldot_multihash_from_bytes(&file_name);
            substrate_lib::substrate_multihash_from_bytes(&file_name);
            gossamer_lib::gossamer_multihash_from_bytes(&file_name);
        }
        Targets::DecodeBabePreDigest => {
            smoldot_lib::smoldot_decode_babepredigest(&file_name);
            substrate_lib::substrate_decode_babepredigest(&file_name);
            gossamer_lib::gossamer_decode_babepredigest(&file_name);
        }
        Targets::PublicKeyFromProtobufEncoding => {
            smoldot_lib::smoldot_publickey_from_protobuf_encoding(&file_name);
            substrate_lib::substrate_publickey_from_protobuf_encoding(&file_name);
            gossamer_lib::gossamer_publickey_from_protobuf_encoding(&file_name);
        }
        Targets::PeerIdFromBytes => {
            smoldot_lib::smoldot_peerid_from_bytes(&file_name);
            substrate_lib::substrate_peerid_from_bytes(&file_name);
            gossamer_lib::gossamer_peerid_from_bytes(&file_name);
        }
        Targets::DecodeBabeNextEpoch => {
            smoldot_lib::smoldot_decode_babenextepoch(&file_name);
            substrate_lib::substrate_decode_babenextepoch(&file_name);
            gossamer_lib::gossamer_decode_babenextepoch(&file_name);
        }
        Targets::DecodeHeader => {
            smoldot_lib::smoldot_decode_header(&file_name);
            substrate_lib::substrate_decode_header(&file_name);
            gossamer_lib::gossamer_decode_header(&file_name);
        }
    };
}

fn run_target(client: Clients, target: Targets, file_name: String) -> Result<(), Error> {
    match client {
        Clients::Gossamer => {
            run_gossamer(target, file_name);
        }
        Clients::Substrate => {
            run_substrate(target, file_name);
        }
        Clients::Smoldot => {
            run_smoldot(target, file_name);
        }
        Clients::All => {
            run_all(target, file_name);
        }
    }
    Ok(())
}
