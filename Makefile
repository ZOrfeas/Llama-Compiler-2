.PHONY: default clean distclean
CXX=$(CLANG)++
CXXFLAGS=-Wall -std=c++20
FLEX=flex
BISON=/opt/homebrew/opt/bison/bin/bison

default: all

all: llamac

# Final linking
llamac: lexer.o parser.o
	$(CXX) $(CXXFlAGS) -o llamac lexer.o parser.o

# Auto-generated lexer and parser
lexer.cpp: lexer.l
	$(FLEX) -s -o lexer.cpp lexer.l
parser.hpp parser.cpp: parser.y
	$(BISON) -dv -Wall -o parser.cpp parser.y

# AST dependency management
ast/ast.hpp: ast/parts/*.hpp
	touch ast/ast.hpp

# Object files
lexer.o: lexer.cpp lexer.hpp parser.hpp ast/ast.hpp
parser.o: parser.cpp lexer.hpp ast/ast.hpp

clean:
	$(RM) llamac

distclean: clean
	$(RM) lexer.cpp parser.hhp parser.cpp