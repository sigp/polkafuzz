## polkafuzz_v1

This tool help to find logic bug using differential fuzzing accross multiple Polkadot client implementation.

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

Build ```Gossamer``` dependencies:

```
make build-gfuzz
```

Build ```polkafuzz_v1```:

```
make build
```

```
./polkafuzz_v1 -h

polkafuzz_v1 0.1.0

USAGE:
    polkafuzz_v1 <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    fuzz         Run fuzzer
    help         Prints this message or the help of the given subcommand(s)
    list         List all available fuzzing targets
    reproduce    Reproduce crash with crash file or hardcoded inputs
    test         Test a target with an empty input or hardcoded inputs
```

List all targets:

```
./polkafuzz_v1 list

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
polkafuzz_v1-fuzz 0.1.0
Run fuzzer

USAGE:
    polkafuzz_v1 fuzz <engine> <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <engine>     [possible values: LibFuzzer, LibAFL]
    <target>    Which target to run [possible values: ChainSpec, MultiaddrFromStr, MultiaddrTryFrom,
                MultihashFromBytes, DecodeBabePreDigest, PublicKeyFromProtobufEncoding, PeerIdFromBytes,
                DecodeBabeNextEpoch]
```

Example:
```
./polkafuzz_v1 fuzz LibFuzzer DecodeBabePreDigest

INFO: Running with entropic power schedule (0xFF, 100).
INFO: Seed: 11919153
INFO: Loaded 1 modules   (2893419 inline 8-bit counters): 2893419 [0x55ff81d84d10, 0x55ff8204737b), 
INFO: Loaded 1 PC tables (2893419 PCs): 2893419 [0x55ff82047380,0x55ff84c6da30), 
INFO:        0 files found in $HOME/workspace/polkafuzz/polkafuzz_v1/fuzz/corpus/decode_babepredigest_libfuzzer
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes
INFO: A corpus is not provided, starting from an empty corpus
#2	INITED cov: 49 ft: 50 corp: 1/1b exec/s: 0 rss: 178Mb
	NEW_FUNC[1/2]: 0x55ff7746ed60  (/home/armagan/workspace/polkafuzz/polkafuzz_v1/fuzz/target/x86_64-unknown-linux-gnu/release/decode_babepredigest_libfuzzer+0x512ed60) (BuildId: fa3a31f567570ef3ad445ab02d0192a4966f6383)
	NEW_FUNC[2/2]: 0x55ff77494370  (/home/armagan/workspace/polkafuzz/polkafuzz_v1/fuzz/target/x86_64-unknown-linux-gnu/release/decode_babepredigest_libfuzzer+0x5154370) (BuildId: fa3a31f567570ef3ad445ab02d0192a4966f6383)
#26	NEW    cov: 64 ft: 67 corp: 2/2b lim: 4 exec/s: 0 rss: 179Mb L: 1/1 MS: 4 InsertByte-CopyPart-EraseBytes-ChangeBinInt-
	NEW_FUNC[1/1]: 0x55ff77470550  (/home/armagan/workspace/polkafuzz/polkafuzz_v1/fuzz/target/x86_64-unknown-linux-gnu/release/decode_babepredigest_libfuzzer+0x5130550) (BuildId: fa3a31f567570ef3ad445ab02d0192a4966f6383)
#30	NEW    cov: 72 ft: 76 corp: 3/3b lim: 4 exec/s: 0 rss: 180Mb L: 1/1 MS: 4 ShuffleBytes-ShuffleBytes-ShuffleBytes-ChangeBinInt-
	NEW_FUNC[1/1]: 0x55ff7746d570  (/home/armagan/workspace/polkafuzz/polkafuzz_v1/fuzz/target/x86_64-unknown-linux-gnu/release/decode_babepredigest_libfuzzer+0x512d570) (BuildId: fa3a31f567570ef3ad445ab02d0192a4966f6383)
#112	NEW    cov: 80 ft: 84 corp: 4/6b lim: 4 exec/s: 0 rss: 180Mb L: 3/3 MS: 2 ChangeByte-CMP- DE: "\001\000"-
#125	REDUCE cov: 80 ft: 84 corp: 4/5b lim: 4 exec/s: 0 rss: 180Mb L: 2/2 MS: 3 PersAutoDict-ChangeByte-EraseBytes- DE: "\001\000"-
	NEW_FUNC[1/1]: 0x55ff774730c0  (/home/armagan/workspace/polkafuzz/polkafuzz_v1/fuzz/target/x86_64-unknown-linux-gnu/release/decode_babepredigest_libfuzzer+0x51330c0) (BuildId: fa3a31f567570ef3ad445ab02d0192a4966f6383)
#387	NEW    cov: 87 ft: 98 corp: 5/10b lim: 6 exec/s: 0 rss: 180Mb L: 5/5 MS: 2 CrossOver-CMP- DE: "\377\377"-
#388	REDUCE cov: 89 ft: 100 corp: 6/16b lim: 6 exec/s: 0 rss: 180Mb L: 6/6 MS: 1 InsertRepeatedBytes-
#397	REDUCE cov: 89 ft: 100 corp: 6/15b lim: 6 exec/s: 0 rss: 180Mb L: 5/5 MS: 4 PersAutoDict-CrossOver-ChangeBit-EraseBytes- DE: "\001\000"-
#490	NEW    cov: 91 ft: 103 corp: 7/20b lim: 6 exec/s: 0 rss: 180Mb L: 5/5 MS: 3 CopyPart-CopyPart-InsertByte-
```
