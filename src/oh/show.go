package oh


import (
	"os"

	// read "github.com/abdomia/oh/src/reader"
	"github.com/olekukonko/tablewriter"
	"github.com/olekukonko/tablewriter/renderer"
	"github.com/olekukonko/tablewriter/tw"
)

func ShowTable(file string) {
	table, _ := tablewriter.NewCSV(
		os.Stdout,
		file,
		true,
		tablewriter.WithRenderer(
			renderer.NewBlueprint(
				tw.Rendition{
					Symbols: tw.NewSymbols(tw.StyleRounded),
				},
				)))
	table.Render()
}

