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
"%" return MODULO;
"(" return LPAREN;
")" return RPAREN;
"==" return EQ;
"!=" return NEQ;
"<" return LT;
">" return GT;
"<=" return LE;
">=" return GE;
"=" return ASSIGN;

"{" return LBRACE;
"}" return RBRACE;

"if" return IF;
"else" return ELSE;
"do" return DO;
"end" return END;

    /* literals */
([0-9]+) {
    yylval.numberLiteral = strdup(yytext);
    return INTEGER_LITERAL;
}

    /* separators */
"\n" return NEWLINE;

    /* identifiers and keywords
    starts with a letter, followed by any number of letters or digits */
([a-zA-Z][a-zA-Z0-9]*) {
    yylval.strLiteral = strdup(yytext);
    return WORD;
}

%%
