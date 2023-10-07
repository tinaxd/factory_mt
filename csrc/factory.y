%{
#include <stdio.h>
#include <stdlib.h>
#include "ast.h"
#define YYDEBUG 1

int yylex();
int yyerror(const char *s);

extern char *yytext;

extern Statement *top_stmt;
%}

%define parse.error verbose
%define parse.trace

%union {
    char *numberLiteral;
    char *strLiteral;
    char *ident;

    Expression *expr;
    Statement *stmt;

    BinaryOperator binop;
}

%token <numberLiteral> INTEGER_LITERAL FLOAT_LITERAL
%token <strLiteral> STRING_LITERAL
%token <ident> WORD
%token LBRACE RBRACE LPAREN RPAREN NEWLINE SEMICOLON COMMA COLON
%token PLUS MINUS TIMES DIVIDE MODULO
%token EQ NEQ LT GT LE GE
%token ASSIGN
%token AND OR NOT
%token IF ELSE WHILE FOR
%token END DO
%token RETURN

%type <expr> expression add_expression product_expression elementary_expression literal_expression cmp_expression
%type <binop> add_operator product_operator cmp_operator
%type <stmt> statement assignment expression_stmt block_statement block_stmt_elements conditional_statement
%type <stmt> program

%%

program: statement {
    //print_expr($1);
    top_stmt = $1;
};

block_stmt_elements:
    statement {
        $$ = $1;
    }
    | block_stmt_elements NEWLINE statement {
        append_block_statement($1, $3);
        $$ = $1;
    };

block_statement:
    DO block_stmt_elements END {
        $$ = make_block_statement($2);
    };

conditional_statement:
    IF expression block_statement {
        $$ = make_cond2($2, $3);
    }
    | IF expression block_statement ELSE block_statement {
        $$ = make_cond3($2, $3, $5);
    };

statement:
    block_statement
    | conditional_statement
    | assignment
    | expression_stmt;

assignment: WORD ASSIGN expression {
    //printf("assignment word: %s\n", $1);
    $$ = make_assign_statement($1, $3);
};

expression_stmt: expression {
    $$ = make_expr_statement($1);
};

expression: cmp_expression {
    $$ = $1;
};

cmp_expression:
    cmp_expression cmp_operator add_expression {
        $$ = make_bin_expr($1, $2, $3);
    }
    | add_expression {
        $$ = $1;
    };

add_expression:
    add_expression add_operator product_expression {
        $$ = make_bin_expr($1, $2, $3);
    }
    | product_expression {
        $$ = $1;
    };

product_expression:
    product_expression product_operator elementary_expression {
        $$ = make_bin_expr($1, $2, $3);
    }
    | elementary_expression {
        $$ = $1;
    };

cmp_operator: EQ {
    $$ = BINOP_EQ;
} | NEQ {
    $$ = BINOP_NEQ;
} | LT {
    $$ = BINOP_LT;
} | GT {
    $$ = BINOP_GT;
} | LE {
    $$ = BINOP_LE;
} | GE {
    $$ = BINOP_GE;
};

add_operator: PLUS {
    $$ = BINOP_PLUS;
} | MINUS {
    $$ = BINOP_MINUS;
};

product_operator: TIMES {
    $$ = BINOP_TIMES;
} | DIVIDE {
    $$ = BINOP_DIVIDE;
} | MODULO {
    $$ = BINOP_MODULO;
};

elementary_expression:
    LPAREN expression RPAREN {
        $$ = $2;
    }
    | literal_expression {
        $$ = $1;
    }
    | WORD {
        $$ = make_name_expr($1);
    }

literal_expression: INTEGER_LITERAL {
    int value = atoi(yytext);
    $$ = make_int_literal(value);
};

%%

int yyerror(const char *str)
{
    extern char *yytext;
    fprintf(stderr, "parse error: %s\n", str);
    return 0;
}
