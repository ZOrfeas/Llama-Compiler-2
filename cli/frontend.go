package cli

import (
	"errors"
)

// var frontendCmd = &cobra.Command{
// 	Use:   "frontend",
// 	Short: "Configure the frontend of the compiler",
// 	Long:  `The frontend of the compiler handles input, lexing, parsing, semantic analysis and intermediate code generation`,
// 	Args:  cobra.MinimumNArgs(1),
// 	Run: func(cmd *cobra.Command, args []string) {
// 		// TODO: Default operation is to fully run the frontend
// 	},
// }
// var frontendPrintCmd = &cobra.Command{
// 	Use:   "print",
// 	Short: "Print the specified intermediate output of the frontend",
// 	Run: func(cmd *cobra.Command, args []string) {
// 		// TODO: Default operation is to print nothing
// 	},
// }

func init() {
	// frontendCmd.AddCommand(frontendPrintCmd)
	// rootCmd.AddCommand(frontendCmd)

	rootCmd.Flags().Var(&StopAfterFlag, "only", "Stop after the specified stage of the frontend (preprocess, lex, parse, sema, irgen)")
	rootCmd.Flags().Lookup("only").Shorthand = "s"

	rootCmd.Flags().StringVar(&PrintSource, "print-source", "", "Print the input text after preprocessing")
	rootCmd.Flags().StringVar(&PrintTokens, "print-tokens", "", "Print the tokens after lexing")
	rootCmd.Flags().StringVar(&PrintAst, "print-ast", "", "Print the AST after parsing")
	rootCmd.Flags().StringVar(&PrintTypes, "print-types", "", "Print the types after semantic analysis")
	rootCmd.Flags().StringVar(&PrintIr, "print-ir", "", "Print the IR after intermediate code generation")
	rootCmd.Flags().Lookup("print-source").NoOptDefVal = "stdout"
	rootCmd.Flags().Lookup("print-tokens").NoOptDefVal = "stdout"
	rootCmd.Flags().Lookup("print-ast").NoOptDefVal = "stdout"
	rootCmd.Flags().Lookup("print-types").NoOptDefVal = "stdout"
	rootCmd.Flags().Lookup("print-ir").NoOptDefVal = "stdout"
}

type StopAfter string

const (
	Preprocess StopAfter = "preprocess"
	Lex        StopAfter = "lex"
	Parse      StopAfter = "parse"
	Sema       StopAfter = "sema"
	IrGen      StopAfter = "irgen"
)

func (sa *StopAfter) String() string {
	return string(*sa)
}
func (sa *StopAfter) Set(value string) error {
	switch value {
	case "preprocess", "lex", "parse", "sema", "irgen":
		*sa = StopAfter(value)
		return nil
	default:
		return errors.New("must be one of: preprocess, lex, parse, sema, irgen")
	}
}
func (sa *StopAfter) Type() string {
	return "string"
}

var (
	StopAfterFlag StopAfter
	PrintSource   string
	PrintTokens   string
	PrintAst      string
	PrintTypes    string
	PrintIr       string
)
