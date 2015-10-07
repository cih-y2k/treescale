package tree_graph

import (
	"encoding/binary"
	"math/big"
	"tree_lib"
)

type Path struct {
	From 			string
	Nodes			[]string				`json:"node_paths" toml:"node_paths" yaml:"node_paths"`
	Groups			[]string				`json:"groups" toml:"groups" yaml:"groups"`
	Tags			[]string				`json:"tags" toml:"tags" yaml:"tags"`
	Path 			*big.Int
}

var (
	CalcPath			func (*Path) (*big.Int, tree_lib.TreeError)
	GetPathValue		func (*Path) (*big.Int, tree_lib.TreeError)
)

func PathValueFromMessage(msg []byte) (body_index int, p *big.Int) {
	// First 4 bytes in message is a length of json encoded Path
	path_len := int(binary.LittleEndian.Uint32(msg[:4]))
	p = big.NewInt(0)
	p.SetBytes(msg[4:path_len+4])
	body_index = 4 + path_len
	return
}

func (p *Path) GetValue() (*big.Int, tree_lib.TreeError) {
	return GetPathValue(p)
}

func (p *Path) CalculatePath() (*big.Int, tree_lib.TreeError) {
	return CalcPath(p)
}