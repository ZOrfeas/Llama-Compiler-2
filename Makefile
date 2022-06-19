.PHONY: default clean distclean
CXX=$(CLANG)++
CXXFLAGS=-Wall -std=c++20
FLEX=flex
BISON=/opt/homebrew/opt/bison/bin/bison

BUILD=./build
OBJS=lexer.o parser.o ast-print.o typesystem.o error.o main.o
OBJS_PATHS=$(patsubst %,$(BUILD)/%,$(OBJS))

default: all

all: llamac

# Final linking
llamac: $(OBJS_PATHS)
	$(CXX) $(CXXFlAGS) -o llamac $^

# Auto-generated lexer and parser
lexer.cpp: lexer.l
	$(FLEX) -s -o lexer.cpp lexer.l
parser.hpp parser.cpp: parser.y
	$(BISON) -dv -Wall -o parser.cpp parser.y

# AST dependency management
ast/ast.hpp: ast/parts/*.hpp
	touch ast/ast.hpp

# Object files
$(BUILD)/lexer.o: lexer.cpp lexer.hpp parser.hpp ast/ast.hpp
$(BUILD)/parser.o: parser.cpp lexer.hpp ast/ast.hpp
$(BUILD)/ast-print.o: passes/print/ast-print.cpp passes/print/ast-print.hpp ast/ast.hpp
$(BUILD)/main.o: main.cpp parser.hpp ast/forward.hpp passes/print/ast-print.hpp
$(BUILD)/typesystem.o: typesystem/typesystem.cpp typesystem/core.hpp typesystem/utils.hpp
$(BUILD)/error.o: error/error.cpp error/error.hpp

# Grouping of rule-types with same recipe
$(BUILD)/%.o: passes/*/%.cpp
$(BUILD)/%.o: %/%.cpp
$(BUILD)/%.o: %.cpp
$(BUILD)/%.o:
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	$(RM) llamac

distclean: clean
	$(RM) lexer.cpp parser.hhp parser.cpp $(BUILD)/*.o