package main

import "C"
import (
	"encoding/json"
	"fmt"
	"github.com/ChainSafe/gossamer/dot/types"
	"github.com/ChainSafe/gossamer/lib/genesis"
	"github.com/ChainSafe/gossamer/pkg/scale"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/peer"
	ma "github.com/multiformats/go-multiaddr"
	mh "github.com/multiformats/go-multihash"
	"reflect"
	"strings"
	"unsafe"
)

func main() {}

//export glib_chain_spec_from_json_bytes
func glib_chain_spec_from_json_bytes(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	g := new(genesis.Genesis)
	err := json.Unmarshal(data, g)
	if err != nil {
		fmt.Println("[-] ChainSpec json.Unmarshal result:", err)
	} else {
		fmt.Println("[+] ChainSpec json.Unmarshal result:", g)
	}
}

//export glib_multiaddr_from_str
func glib_multiaddr_from_str(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	data_str := strings.Split(string(data[:]), "\n")
	ret, err := ma.NewMultiaddr(data_str[0])
	if err != nil {
		fmt.Println("[-] Multiaddr ma.NewMultiaddr result:", err)
	} else {
		fmt.Println("[+] Multiaddr ma.NewMultiaddr result:", ret)
	}
}

//export glib_multiaddr_try_from
func glib_multiaddr_try_from(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	ret, err := ma.NewMultiaddrBytes(data)
	if err != nil {
		fmt.Println("[-] Multiaddr ma.NewMultiaddrBytes result:", err)
	} else {
		fmt.Println("[+] Multiaddr ma.NewMultiaddr result:", ret)
	}

}

//export glib_multihash_from_bytes
func glib_multihash_from_bytes(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	_, ret, err := mh.MHFromBytes(data)
	if err != nil {
		fmt.Println("[-] Multihash mh.MHFromBytes result:", err)
	} else {
		fmt.Println("[+] Multihash mh.MHFromBytes result:", ret)
	}
}

//export glib_decode_babepredigest
func glib_decode_babepredigest(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	ret, err := types.DecodeBabePreDigest(data)
	if err != nil {
		fmt.Println("[-] BabePreDigest DecodeBabePreDigest result:", err)
	} else {
		fmt.Println("[+] BabePreDigest DecodeBabePreDigest result:", ret)
	}
}

//export glib_publickey_from_protobuf_encoding
func glib_publickey_from_protobuf_encoding(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	ret, err := crypto.UnmarshalPublicKey(data)
	if err != nil {
		fmt.Println("[-] PublicKey UnmarshalPublicKey result:", err)
	} else {
		fmt.Println("[+] PublicKey UnmarshalPublicKey result:", ret)
	}
}

//export glib_peerid_from_bytes
func glib_peerid_from_bytes(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	ret, err := peer.IDFromBytes(data)
	if err != nil {
		fmt.Println("[-] PeerId IDFromBytes result:", err)
	} else {
		fmt.Println("[+] PeerId IDFromBytes result:", ret)
	}
}

//export glib_decode_babenextepoch
func glib_decode_babenextepoch(data_ptr unsafe.Pointer, data_size int) {
	fmt.Println("[+] Gossamer Result:")
	var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	dec := types.NextEpochData{}
	err := scale.Unmarshal(data, &dec)
	if err != nil {
		fmt.Println("[-] BabeNextEpoch scale.Unmarshal result:", err)
	} else {
		fmt.Println("[+] BabeNextEpoch scale.Unmarshal result:", dec)
	}
}

//export glib_decode_header
func glib_decode_header(data_ptr unsafe.Pointer, data_size int) {	
	fmt.Println("[+] Gossamer Result:")
    var data []byte
	sh := (*reflect.SliceHeader)(unsafe.Pointer(&data))
	sh.Data = uintptr(data_ptr)
	sh.Len = data_size
	sh.Cap = data_size
	dec := types.NewEmptyHeader()
	err := scale.Unmarshal(data, &dec)
	if err != nil {
		fmt.Println("[-] Header scale.Unmarshal result:", err)
	} else {
		fmt.Println("[+] Header scale.Unmarshal result:", dec)
	}
}
