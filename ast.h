typedef enum ExpressionType
{
    EXPR_BINARY,
    EXPR_LITERAL,
} ExpressionType;

typedef enum BinaryOperator
{
    BINOP_PLUS,
    BINOP_MINUS,
    BINOP_TIMES,
    BINOP_DIVIDE,
    BINOP_MODULO,
} BinaryOperator;

struct BinaryExpression;
struct LiteralExpression;

typedef struct Expression
{
    int type;
    union
    {
        struct BinaryExpression *bin;
        struct LiteralExpression *lit;
    } expr;
} Expression;

typedef struct BinaryExpression
{
    BinaryOperator op;
    Expression *left;
    Expression *right;
} BinaryExpression;

typedef struct LiteralExpression
{
    int type;
    union
    {
        int int_value;
        float float_value;
    } value;
} LiteralExpression;

Expression *make_int_literal(int value);
Expression *make_bin_expr(Expression *left, BinaryOperator op, Expression *right);

void print_expr(Expression *expr);