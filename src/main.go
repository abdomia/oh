package main

import (
	"os"

	cmds "github.com/abdomia/oh/src/oh"
)

func main() {
	if e := cmds.Oh.Execute(); e != nil {
		os.Exit(1)
	}
}

