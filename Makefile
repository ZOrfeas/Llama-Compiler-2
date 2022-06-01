.PHONY: default clean distclean
CXX=$(CLANG)++
CXXFLAGS=-Wall -std=c++20
FLEX=flex
BISON=/opt/homebrew/opt/bison/bin/bison

BUILD=./build

default: all

all: llamac

# Final linking
llamac: $(BUILD)/lexer.o $(BUILD)/parser.o $(BUILD)/ast-print.o $(BUILD)/main.o
	$(CXX) $(CXXFlAGS) -o llamac \
	$(BUILD)/lexer.o \
	$(BUILD)/parser.o \
	$(BUILD)/ast-print.o \
	$(BUILD)/main.o


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
	$(CXX) $(CXXFLAGS) -c -o $@ $<
$(BUILD)/parser.o: parser.cpp lexer.hpp ast/ast.hpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<
$(BUILD)/ast-print.o: passes/print/ast-print.cpp passes/print/ast-print.hpp ast/ast.hpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<
$(BUILD)/main.o: main.cpp parser.hpp ast/forward.hpp passes/print/ast-print.hpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<

clean:
	$(RM) llamac

distclean: clean
	$(RM) lexer.cpp parser.hhp parser.cpp $(BUILD)/*.o