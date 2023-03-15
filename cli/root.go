package cli

import (
	"errors"
	"fmt"
	"os"

	"github.com/ZOrfeas/Llama-Compiler-2/tree/orf/go/parse"
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
		scanner, err := parse.NewFileScanner(args[0])
		if err != nil {
			return err
		}
		for line, exists := scanner.Next(); exists; line, exists = scanner.Next() {
			fmt.Println(line)
		}
		return nil
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func init() {

}
