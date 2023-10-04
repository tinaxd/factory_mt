%{
#include <stdio.h>
#include "ast.h"
#include "factory.tab.h"

      int yywrap(void){
          return 1;
}
%}

%%

    /* operators */
"+" return PLUS;
"-" return MINUS;
"*" return TIMES;
"/" return DIVIDE;
"(" return LPAREN;
")" return RPAREN;

    /* literals */
([0-9]+) {
    yylval.numberLiteral = yytext;
    return INTEGER_LITERAL;
}

    /* separators */
"\n" return NEWLINE;

    /* identifiers and keywords
    starts with a letter, followed by any number of letters or digits */
([a-zA-Z][a-zA-Z0-9]*) {
    yylval.strLiteral = yytext;
    return WORD;
}

%%
