package main

import "C"
import (
	"reflect"
	"unsafe"
	"encoding/json"

	"github.com/ChainSafe/gossamer/lib/genesis"
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