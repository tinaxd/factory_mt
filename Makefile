factory.tab.c factory.tab.h parser.log: factory.y
	bison -d -r all --report-file=parser.log factory.y

lex.yy.c: factory.l factory.tab.h
	flex factory.l

factory: lex.yy.c factory.tab.c ast.cpp compiler.cpp consttable.cpp vm.cpp object.cpp main.cpp
	c++ -Wall -Wextra -O0 -g3 -o factory factory.tab.c lex.yy.c ast.cpp compiler.cpp consttable.cpp vm.cpp object.cpp main.cpp
