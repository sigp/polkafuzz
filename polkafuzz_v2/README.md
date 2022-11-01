## polkafuzz_v2

This tool help to find bugs using coverage guided fuzzing accross multiple Polkadot client implementation.

## Setup and Installation

Install rust fuzzers:

```
cargo install -f cargo-fuzz

cargo install -f cargo-libafl
```

Update the submodules:

```
make update
```

Build ```polkafuzz_v2```:

```
make build
```

```
./polkafuzz_v2 -h

polkafuzz_v2 0.1.0

USAGE:
    polkafuzz_v2 <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    fuzz    Run fuzzer
    help    Prints this message or the help of the given subcommand(s)
    list    List all available fuzzing targets
```

List all targets:

```
./polkafuzz_v2 list

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

Run the fuzzers:

```
./polkafuzz_v2 fuzz --help

polkafuzz_v2-fuzz 0.1.0
Run fuzzer

USAGE:
    polkafuzz_v2 fuzz <client> <engine> <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <client>     [possible values: Gossamer, Smoldot, Substrate]
    <engine>     [possible values: LibFuzzer, LibAFL]
    <target>    Which target to run [possible values: ChainSpec, MultiaddrFromStr, MultiaddrTryFrom,
                MultihashFromBytes, DecodeBabePreDigest, PublicKeyFromProtobufEncoding, PeerIdFromBytes,
                DecodeBabeNextEpoch]
```

Example:
```
/polkafuzz_v2 fuzz smoldot libfuzzer DecodeBabeNextEpoch

INFO: Running with entropic power schedule (0xFF, 100).
INFO: Seed: 745857078
INFO: Loaded 1 modules   (1743998 inline 8-bit counters): 1743998 [0x55e32fd465b0, 0x55e32fef022e),
INFO: Loaded 1 PC tables (1743998 PCs): 1743998 [0x55e32fef0230,0x55e33198ca10),
INFO:       16 files found in $HOME/workspace/polkafuzz/polkafuzz_v2/fuzzers/smoldot/fuzz/corpus/decode_babenextepoch_libfuzzer
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes
INFO: seed corpus: files: 16 min: 1b max: 33b total: 124b rss: 110Mb
#17	INITED cov: 71 ft: 72 corp: 15/116b exec/s: 0 rss: 112Mb
#103	NEW    cov: 72 ft: 73 corp: 16/119b lim: 33 exec/s: 0 rss: 112Mb L: 3/33 MS: 1 InsertByte-
#734	REDUCE cov: 72 ft: 73 corp: 16/118b lim: 38 exec/s: 0 rss: 112Mb L: 2/33 MS: 1 EraseBytes-
#1126	REDUCE cov: 72 ft: 73 corp: 16/117b lim: 38 exec/s: 0 rss: 112Mb L: 1/33 MS: 2 ChangeByte-EraseBytes-
#6654	NEW    cov: 73 ft: 74 corp: 17/138b lim: 92 exec/s: 0 rss: 113Mb L: 21/33 MS: 3 CopyPart-ChangeByte-ChangeBit-
thread '<unnamed>' panicked at 'attempt to multiply with overflow', $HOME/workspace/polkafuzz/clients/smoldot/src/header/babe.rs:150:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
