.PHONY: default clean distclean
CXX=$(CLANG)++
CLI11_INCLUDE=-I/opt/homebrew/Cellar/cli11/2.2.0/include
CXXFLAGS=-Wall -std=c++20 $(CLI11_INCLUDE)
FLEX=flex
BISON=/opt/homebrew/opt/bison/bin/bison

BUILD=./build
NON_MAIN_OBJS=lexer.o parser.o ast-print.o typesystem.o error.o cli.o
OBJS=$(NON_MAIN_OBJS) main.o
NON_MAIN_OBJS_PATHS=$(patsubst %,$(BUILD)/%,$(NON_MAIN_OBJS))
OBJS_PATHS=$(patsubst %,$(BUILD)/%,$(OBJS))

default: all

all: llamac

# $(CXX) $(CXXFLAGS) -c tests/drafts.cpp -o $(BUILD)/drafts.o
test: $(NON_MAIN_OBJS_PATHS) tests/drafts.cpp
	$(CXX) $(CXXFLAGS) -o test $^

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
$(BUILD)/ast-print.o: passes/print/ast-print.cpp \
 passes/print/ast-print.hpp ast/ast.hpp
$(BUILD)/main.o: main.cpp parser.hpp \
 ast/forward.hpp passes/print/ast-print.hpp
$(BUILD)/typesystem.o: typesystem/typesystem.cpp \
 typesystem/core.hpp typesystem/types.hpp \
 error/error.hpp utils/utils.hpp
$(BUILD)/error.o: error/error.cpp error/error.hpp
$(BUILD)/cli.o: cli/cli.cpp cli/cli.hpp

# Grouping of rule-types with same recipe
$(BUILD)/%.o: passes/*/%.cpp
$(BUILD)/%.o: %/%.cpp
$(BUILD)/%.o: %.cpp
$(BUILD)/%.o:
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	$(RM) llamac test

distclean: clean
	$(RM) lexer.cpp parser.hhp parser.cpp $(BUILD)/*.o
