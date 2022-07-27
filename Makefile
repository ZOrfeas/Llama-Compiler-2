.PHONY: default clean distclean
CXX=$(CLANG)++

LIBS=cli11 fmt
FLEX_INCLUDE=-I/opt/homebrew/opt/flex/include
CXXFLAGS_INCLUDE=$(shell pkg-config --cflags $(LIBS)) $(FLEX_INCLUDE)
CXXFLAGS=-Wall -std=c++20

CXXFLAGS_LIBS=$(shell pkg-config --libs $(LIBS))

FLEX=/opt/homebrew/Cellar/flex/2.6.4_2/bin/flex
BISON=/opt/homebrew/opt/bison/bin/bison

BUILD=./build
NON_MAIN_OBJS=scanner.o parser.o generator.o typesystem.o cli.o
OBJS=$(NON_MAIN_OBJS) main.o
NON_MAIN_OBJS_PATHS=$(patsubst %,$(BUILD)/%,$(NON_MAIN_OBJS))
OBJS_PATHS=$(patsubst %,$(BUILD)/%,$(OBJS))

default: all

all: llamac

# $(CXX) $(CXXFLAGS) -c tests/drafts.cpp -o $(BUILD)/drafts.o
test: $(NON_MAIN_OBJS_PATHS) tests/drafts.cpp
	$(CXX) $(CXXFLAGS) $(CXXFLAGS_INCLUDE) $(CXXFLAGS_LIBS) -o test $^

# Final linking
llamac: $(OBJS_PATHS)
	$(CXX) $(CXXFlAGS) $(CXXFLAGS_LIBS) -o llamac $^

# Auto-generated scanner and parser
parser/scanner.cpp: parser/scanner.l parser/scanner.hpp ast/ast.hpp parser/parser.hpp
	$(FLEX) -s -o $@ $<
parser/parser.hpp parser/parser.cpp: parser/parser.yy ast/ast.hpp parser/scanner.hpp log/log.hpp
	$(BISON) -dv -Wall -o parser/parser.cpp $<

# Object files
$(BUILD)/scanner.o: parser/scanner.cpp parser/scanner.hpp \
 parser/generator.hpp parser/parser.hpp
$(BUILD)/parser.o: parser/parser.cpp parser/parser.hpp \
 parser/scanner.hpp parser/generator.hpp ast/ast.hpp
$(BUILD)/generator.o: parser/generator.cpp parser/generator.hpp \
 ast/ast.hpp
$(BUILD)/main.o: main.cpp parser/generator.hpp \
 ast/ast.hpp cli/cli.hpp
$(BUILD)/typesystem.o: typesystem/typesystem.cpp \
 typesystem/types.hpp utils/utils.hpp
$(BUILD)/cli.o: cli/cli.cpp cli/cli.hpp

# header dependency management
ast/ast.hpp: ast/parts/*.hpp

typesystem/core.hpp: utils/utils.hpp log/log.hpp typesystem/forward.hpp
typesystem/types.hpp: typesystem/core.hpp
parser/scanner.hpp: ast/forward.hpp


# Grouping of rule-types with same recipe
%.hpp:
	touch $@
# $(BUILD)/%.o: passes/*/%.cpp
# $(BUILD)/%.o: %/%.cpp
# $(BUILD)/%.o: %.cpp
$(BUILD)/%.o:
	$(CXX) $(CXXFLAGS) $(CXXFLAGS_INCLUDE) -c $< -o $@

clean:
	$(RM) llamac test

distclean: clean
	$(RM) scanner.cpp parser.hpp parser.cpp $(BUILD)/*.o \
		parser/scanner.cpp parser/parser.cpp parser/parser.hpp \
		parser/parser.output
