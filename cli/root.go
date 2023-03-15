package cli

import (
	"errors"
	"fmt"
	"os"

	"github.com/ZOrfeas/Llama-Compiler-2/tree/orf/go/parse"
	"github.com/ZOrfeas/Llama-Compiler-2/tree/orf/go/utils"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "llamac [file]",
	Short: "llamac is a compiler for the Llama programming language",
	Long:  `It implements the specification provided in https://courses.softlab.ntua.gr/compilers/2021a/llama2021.pdf and adds a few extra goodies`,
	Args: func(cmd *cobra.Command, args []string) error {
		if err := cobra.MinimumNArgs(1)(cmd, args); err != nil {
			return err
		}
		if _, err := os.Stat(args[0]); errors.Is(err, os.ErrNotExist) {
			return err
		}
		return nil
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		// TODO: Default operation is to fully run the compiler
		var scanner utils.Generator[parse.ScanEvent] = parse.NewFileScanner(args[0])
		if PrintSource != "" {
			scanner = utils.GenMap(
				scanner,
				make_writer(PrintSource, func(event parse.ScanEvent) string {
					return event.SourceLine()
				}),
			)
		}
		if StopAfterFlag == Preprocess {
			utils.GenUnroll(scanner)
		}
		// for line, exists := scanner.Next(); exists; line, exists = scanner.Next() {
		// 	text := line.SourceLine()
		// 	if text != "" {
		// 		fmt.Println(text)
		// 	}
		// }
		return nil
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func make_writer[Item any](path string, fn func(Item) string) func(Item) Item {
	var file *os.File
	if path == "stdout" {
		file = os.Stdout
	} else {
		filetmp, err := os.Create(path)
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		file = filetmp
	}
	return func(item Item) Item {
		str := fn(item)
		if str != "" {
			fmt.Fprintln(file, str)
		}
		return item
	}
}

func init() {

}
