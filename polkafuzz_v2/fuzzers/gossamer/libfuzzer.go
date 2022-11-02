package gossamer

import (
	"encoding/json"
	"github.com/ChainSafe/gossamer/dot/types"
	"github.com/ChainSafe/gossamer/lib/genesis"
	"github.com/ChainSafe/gossamer/pkg/scale"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/peer"
	ma "github.com/multiformats/go-multiaddr"
	mh "github.com/multiformats/go-multihash"
)

const (
	fuzzInteresting = 1
	fuzzNormal      = 0
	fuzzDiscard     = -1
)

func Fuzz_chain_spec(data []byte) int {
	g := new(genesis.Genesis)
	err := json.Unmarshal(data, g)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_multiaddr_from_str(data []byte) int {
	_, err := ma.NewMultiaddr(string(data[:]))
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_multiaddr_try_from(data []byte) int {
	_, err := ma.NewMultiaddrBytes(data)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_multihash_from_bytes(data []byte) int {
	_, _, err := mh.MHFromBytes(data)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_decode_babepredigest(data []byte) int {
	_, err := types.DecodeBabePreDigest(data)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_publickey_from_protobuf_encoding(data []byte) int {
	_, err := crypto.UnmarshalPublicKey(data)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_peerid_from_bytes(data []byte) int {
	_, err := peer.IDFromBytes(data)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}

func Fuzz_decode_babenextepoch(data []byte) int {
	dec := types.NextEpochData{}
	err := scale.Unmarshal(data, &dec)
	if err != nil {
		return fuzzNormal
	}
	return fuzzInteresting
}
