package main

import "C"
import (
	"encoding/json"
	"reflect"
	"strings"
	"unsafe"

	"github.com/ChainSafe/gossamer/dot/types"
	"github.com/ChainSafe/gossamer/lib/genesis"
	"github.com/ChainSafe/gossamer/pkg/scale"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/peer"
	ma "github.com/multiformats/go-multiaddr"
	mh "github.com/multiformats/go-multihash"
)

func main() {}

//export gfuzz_genesis_json_from_bytes
func gfuzz_genesis_json_from_bytes(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	g := new(genesis.Genesis)
	err := json.Unmarshal(data, g)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_new_multiaddr_bytes
func gfuzz_new_multiaddr_bytes(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, err := ma.NewMultiaddrBytes(data)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_new_multiaddr
func gfuzz_new_multiaddr(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	data_str := strings.Split(string(data[:]), "\n")
	_, err := ma.NewMultiaddr(data_str[0])
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_decode_babepredigest
func gfuzz_decode_babepredigest(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, err := types.DecodeBabePreDigest(data)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_publickey_from_proto
func gfuzz_publickey_from_proto(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, err := crypto.UnmarshalPublicKey(data)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_peerid_from_bytes
func gfuzz_peerid_from_bytes(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, err := peer.IDFromBytes(data)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_multihash_from_bytes
func gfuzz_multihash_from_bytes(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, _, err := mh.MHFromBytes(data)
	if err != nil {
		return false
	}
	return true
}

//export gfuzz_decode_babenextepoch
func gfuzz_decode_babenextepoch(data_ptr unsafe.Pointer, data_size int) bool {
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	dec := types.NextEpochData{}
	err := scale.Unmarshal(data, &dec)
	if err != nil {
		return false
	}
	return true
}
