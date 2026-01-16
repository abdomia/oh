// Package oh: the Main entry point for this project
package oh

import (
	"fmt"

	read "github.com/abdomia/oh/src/reader"
	"github.com/spf13/cobra"
)

var Oh = &cobra.Command{
	Long: "oh command is all in one toolkit csv processor",
	Short: "csv toolkit",
	Run: RunOh,
}

func RunOh(cmd *cobra.Command, args []string) {
	data := read.ReadFromDisk(args[0])
	fmt.Println(data)

	ShowTable(args[0])
}

