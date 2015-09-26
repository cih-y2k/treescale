package tree_api
import (
	"tree_event"
	"tree_log"
	"tree_node/node_info"
	"tree_lib"
	"fmt"
	"tree_db"
)

const (
	API_NAME_PREFIX		=	"___TREE_API___"
	log_from_node_api	=	"Node API Backend"
)

var (
	EmitApi			func(*tree_event.Event, ...string)tree_lib.TreeError
	EmitToApi		func(*tree_event.Event, ...string)tree_lib.TreeError
)

func init() {
	tree_event.ON(tree_event.ON_API_CONNECTED, func(e *tree_event.Event){
		tree_log.Info(log_from_node_api, "New API client connected -> ", string(e.Data))
	})

	tree_event.ON(tree_event.ON_API_DISCONNECTED, func(e *tree_event.Event){
		tree_log.Info(log_from_node_api, "New API client disconnected -> ", string(e.Data))
	})
}

// Init API node for connection to targets
func API_INIT(targets...string) bool {
	var err tree_lib.TreeError
	err.From = tree_lib.FROM_API_INIT
	if len(targets) == 0 {
		tree_log.Error(err.From,"For running API client you need to specify target node(s) to connect")
		return false
	}
	for _, n :=range targets {
		node_info.ChildsNodeInfo[n], err = tree_db.GetNodeInfo(n)
		if !err.IsNull() {
			tree_log.Error(err.From, fmt.Sprintf("Unable Getting target (%s) node info from Node database, ", n), err.Error())
			return false
		}
	}

	node_info.CurrentNodeInfo = node_info.NodeInfo{
		Name: fmt.Sprintf("%s|%s", API_NAME_PREFIX, tree_lib.RandomString(10)),
		Childs: targets,
	}

	return true
}