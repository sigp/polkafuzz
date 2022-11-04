## reproducer

This tool help to replay samples accross multiple Polkadot client implementation.

When samples or crash artifacts is provided, reproducer will process them through all Polkadot clients.

The goal is to detect execution differences between all Polkadot implementation and reproduce crashes to help with debugging and triaging.

## Setup

Update the submodules:

```
make update
```

Build ```Gossamer``` dependencies:

```
make build-gfuzz
```

Build ```reproducer```:

```
make build
```

```
./reproducer -h

reproducer 0.1.0

USAGE:
    reproducer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    List all available targets
    run     Run target function over multiple Polkadot client implementation
```

List all targets:

```
./reproducer list

[+] List of available targets:
    ChainSpec
    MultiaddrFromStr
    MultiaddrTryFrom
    MultihashFromBytes
    DecodeBabePreDigest
    PublicKeyFromProtobufEncoding
    PeerIdFromBytes
    DecodeBabeNextEpoch
```

Run the reproducer:

```
./reproducer run --help

reproducer-run 0.1.0
Run target function over multiple Polkadot client implementation

USAGE:
    reproducer run <client> <target> <file-name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <client>        [possible values: Gossamer, Smoldot, Substrate, All]
    <target>        Which target to run [possible values: ChainSpec, MultiaddrFromStr, MultiaddrTryFrom,
                    MultihashFromBytes, DecodeBabePreDigest, PublicKeyFromProtobufEncoding, PeerIdFromBytes,
                    DecodeBabeNextEpoch]
    <file-name>     Crash or sample file
```

Example:

```
./reproducer run all PublicKeyFromProtobufEncoding $HOME/workspace/polkafuzz/corpora/publickey_from_protobuf_encoding/9b7514435d727defaeac3aace81da1e53110597b

[+] Smoldot Result:
[-] PublicKey from_protobuf_encoding result: Err(ProtobufDecodeError)
[+] Substrate Result:
[-] PublicKey from_protobuf_encoding result: Err(DecodingError { msg: "Protobuf", source: Some(DecodeError { description: "invalid wire type: Varint (expected LengthDelimited)", stack: [("PublicKey", "data")] }) })
[+] Gossamer Result:
[-] PublicKey UnmarshalPublicKey result: proto: wrong wireType = 0 for field Data
```
