%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Query processing for LLM inference requests
int yylex(void);
void yyerror(const char *s);

// Query result structures
typedef struct {
    char *field_name;
    char *field_value;
} QueryField;

typedef struct {
    char *model_name;
    char *query_type;
    QueryField *fields;
    int field_count;
    bool streaming;
    int limit;
} QueryResult;

// Global variables for query processing
QueryResult current_query;
char *selected_model = NULL;
bool debug_mode = false;

// Function prototypes
void init_query(void);
void set_model(char *model);
void add_field(char *name, char *value);
void execute_query(void);
void print_results(void);
QueryResult process_inference_query(char *prompt, char *model);
QueryResult process_embedding_query(char *text, char *model);
QueryResult process_status_query(char *agent_id);
%}

%union {
    char *sval;
    int ival;
    double dval;
    bool bval;
}

%token <sval> IDENTIFIER STRING QUOTED_STRING
%token <ival> INTEGER
%token <dval> FLOAT
%token <bval> BOOLEAN

// SQL-like keywords for querying LLM services
%token SELECT FROM WHERE LIMIT ORDER BY GROUP HAVING
%token INSERT INTO VALUES UPDATE SET DELETE
%token MODEL MODELS AGENTS AGENT STATUS METRICS
%token INFERENCE EMBEDDING COMPLETION CHAT
%token PROMPT TEXT TEMPERATURE MAX_TOKENS TOP_P TOP_K
%token STREAM STOP SEED FREQUENCY_PENALTY PRESENCE_PENALTY
%token AND OR NOT NULL IS LIKE IN BETWEEN
%token ASC DESC
%token COUNT SUM AVG MAX MIN
%token INNER LEFT RIGHT FULL OUTER JOIN ON
%token UNION INTERSECT EXCEPT
%token CASE WHEN THEN ELSE END
%token CREATE TABLE INDEX VIEW DROP ALTER
%token PRIMARY KEY FOREIGN REFERENCES UNIQUE

// Operators
%left OR
%left AND
%right NOT
%left '=' '!=' '<' '>' LE GE LIKE IN
%left '+' '-'
%left '*' '/' '%'
%right UMINUS

%type <sval> table_name column_name value_expr string_expr
%type <ival> integer_expr limit_clause
%type <dval> numeric_expr
%type <bval> boolean_expr condition_expr

%%

query_statement:
    select_statement { printf("Executing SELECT query\n"); execute_query(); }
    | insert_statement { printf("Executing INSERT statement\n"); }
    | update_statement { printf("Executing UPDATE statement\n"); }
    | delete_statement { printf("Executing DELETE statement\n"); }
    | inference_query { printf("Processing inference query\n"); }
    | status_query { printf("Processing status query\n"); }
    | model_management { printf("Managing models\n"); }
    ;

select_statement:
    SELECT select_list FROM from_list where_clause group_clause having_clause order_clause limit_clause {
        current_query.limit = $9;
        printf("SELECT with limit %d\n", $9);
    }
    ;

select_list:
    '*' { printf("SELECT all columns\n"); }
    | column_list
    ;

column_list:
    column_name { add_field($1, NULL); }
    | column_list ',' column_name { add_field($3, NULL); }
    | aggregate_function
    | column_list ',' aggregate_function
    ;

aggregate_function:
    COUNT '(' '*' ')' { printf("COUNT(*) aggregate\n"); }
    | COUNT '(' column_name ')' { printf("COUNT(%s) aggregate\n", $3); }
    | SUM '(' column_name ')' { printf("SUM(%s) aggregate\n", $3); }
    | AVG '(' column_name ')' { printf("AVG(%s) aggregate\n", $3); }
    | MAX '(' column_name ')' { printf("MAX(%s) aggregate\n", $3); }
    | MIN '(' column_name ')' { printf("MIN(%s) aggregate\n", $3); }
    ;

from_list:
    table_name { printf("FROM table: %s\n", $1); }
    | from_list ',' table_name { printf("FROM table: %s\n", $3); }
    | join_expression
    ;

join_expression:
    table_name JOIN table_name ON condition_expr {
        printf("INNER JOIN between %s and %s\n", $1, $3);
    }
    | table_name LEFT JOIN table_name ON condition_expr {
        printf("LEFT JOIN between %s and %s\n", $1, $4);
    }
    | table_name RIGHT JOIN table_name ON condition_expr {
        printf("RIGHT JOIN between %s and %s\n", $1, $4);
    }
    ;

where_clause:
    /* empty */ { }
    | WHERE condition_expr { printf("WHERE condition applied\n"); }
    ;

group_clause:
    /* empty */ { }
    | GROUP BY column_list { printf("GROUP BY applied\n"); }
    ;

having_clause:
    /* empty */ { }
    | HAVING condition_expr { printf("HAVING condition applied\n"); }
    ;

order_clause:
    /* empty */ { }
    | ORDER BY order_list { printf("ORDER BY applied\n"); }
    ;

order_list:
    column_name { printf("ORDER BY %s\n", $1); }
    | column_name ASC { printf("ORDER BY %s ASC\n", $1); }
    | column_name DESC { printf("ORDER BY %s DESC\n", $1); }
    | order_list ',' column_name { printf("ORDER BY %s\n", $3); }
    ;

limit_clause:
    /* empty */ { $$ = -1; }
    | LIMIT integer_expr { $$ = $2; }
    ;

// LLM-specific query types
inference_query:
    INFERENCE FROM MODEL string_expr WITH inference_params {
        set_model($4);
        printf("Inference query for model: %s\n", $4);
        current_query = process_inference_query("default_prompt", $4);
    }
    | COMPLETION string_expr FROM MODEL string_expr WITH completion_params {
        set_model($5);
        printf("Completion query: %s from model: %s\n", $2, $5);
    }
    | CHAT string_expr FROM MODEL string_expr WITH chat_params {
        set_model($5);
        printf("Chat query: %s from model: %s\n", $2, $5);
    }
    ;

embedding_query:
    EMBEDDING string_expr FROM MODEL string_expr {
        set_model($5);
        printf("Embedding query for text: %s from model: %s\n", $2, $5);
        current_query = process_embedding_query($2, $5);
    }
    ;

status_query:
    SELECT STATUS FROM AGENT string_expr {
        printf("Status query for agent: %s\n", $5);
        current_query = process_status_query($5);
    }
    | SELECT METRICS FROM MODELS {
        printf("Metrics query for all models\n");
    }
    | SELECT STATUS FROM MODELS WHERE condition_expr {
        printf("Conditional status query for models\n");
    }
    ;

inference_params:
    inference_param
    | inference_params ',' inference_param
    ;

inference_param:
    TEMPERATURE '=' numeric_expr { printf("Temperature: %.2f\n", $3); }
    | MAX_TOKENS '=' integer_expr { printf("Max tokens: %d\n", $3); }
    | TOP_P '=' numeric_expr { printf("Top-p: %.2f\n", $3); }
    | TOP_K '=' integer_expr { printf("Top-k: %d\n", $3); }
    | SEED '=' integer_expr { printf("Seed: %d\n", $3); }
    | STREAM '=' boolean_expr { printf("Streaming: %s\n", $3 ? "true" : "false"); }
    | STOP '=' string_list { printf("Stop sequences set\n"); }
    | FREQUENCY_PENALTY '=' numeric_expr { printf("Frequency penalty: %.2f\n", $3); }
    | PRESENCE_PENALTY '=' numeric_expr { printf("Presence penalty: %.2f\n", $3); }
    ;

completion_params:
    completion_param
    | completion_params ',' completion_param
    ;

completion_param:
    TEMPERATURE '=' numeric_expr { printf("Completion temperature: %.2f\n", $3); }
    | MAX_TOKENS '=' integer_expr { printf("Completion max tokens: %d\n", $3); }
    ;

chat_params:
    chat_param
    | chat_params ',' chat_param
    ;

chat_param:
    TEMPERATURE '=' numeric_expr { printf("Chat temperature: %.2f\n", $3); }
    | MAX_TOKENS '=' integer_expr { printf("Chat max tokens: %d\n", $3); }
    ;

string_list:
    string_expr { printf("String: %s\n", $1); }
    | string_list ',' string_expr { printf("String: %s\n", $3); }
    ;

model_management:
    CREATE MODEL string_expr FROM string_expr {
        printf("Creating model %s from %s\n", $3, $5);
    }
    | DROP MODEL string_expr {
        printf("Dropping model %s\n", $3);
    }
    | ALTER MODEL string_expr SET model_param_list {
        printf("Altering model %s\n", $3);
    }
    ;

model_param_list:
    model_param
    | model_param_list ',' model_param
    ;

model_param:
    IDENTIFIER '=' value_expr {
        printf("Setting %s = %s\n", $1, $3);
        add_field($1, $3);
    }
    ;

// Standard SQL operations
insert_statement:
    INSERT INTO table_name '(' column_list ')' VALUES '(' value_list ')' {
        printf("INSERT INTO %s\n", $3);
    }
    | INSERT INTO table_name VALUES '(' value_list ')' {
        printf("INSERT INTO %s (all columns)\n", $3);
    }
    ;

update_statement:
    UPDATE table_name SET update_list where_clause {
        printf("UPDATE %s\n", $2);
    }
    ;

update_list:
    column_name '=' value_expr {
        printf("SET %s = %s\n", $1, $3);
    }
    | update_list ',' column_name '=' value_expr {
        printf("SET %s = %s\n", $3, $5);
    }
    ;

delete_statement:
    DELETE FROM table_name where_clause {
        printf("DELETE FROM %s\n", $3);
    }
    ;

value_list:
    value_expr { printf("Value: %s\n", $1); }
    | value_list ',' value_expr { printf("Value: %s\n", $3); }
    ;

// Expressions
condition_expr:
    boolean_expr { $$ = $1; }
    | value_expr '=' value_expr { $$ = (strcmp($1, $3) == 0); }
    | value_expr '!=' value_expr { $$ = (strcmp($1, $3) != 0); }
    | numeric_expr '<' numeric_expr { $$ = ($1 < $3); }
    | numeric_expr '>' numeric_expr { $$ = ($1 > $3); }
    | numeric_expr LE numeric_expr { $$ = ($1 <= $3); }
    | numeric_expr GE numeric_expr { $$ = ($1 >= $3); }
    | condition_expr AND condition_expr { $$ = $1 && $3; }
    | condition_expr OR condition_expr { $$ = $1 || $3; }
    | NOT condition_expr { $$ = !$2; }
    | '(' condition_expr ')' { $$ = $2; }
    | column_name IS NULL { printf("%s IS NULL\n", $1); $$ = true; }
    | column_name IS NOT NULL { printf("%s IS NOT NULL\n", $1); $$ = true; }
    | value_expr LIKE string_expr { printf("%s LIKE %s\n", $1, $3); $$ = true; }
    | value_expr IN '(' value_list ')' { printf("%s IN (...)\n", $1); $$ = true; }
    | value_expr BETWEEN value_expr AND value_expr { printf("%s BETWEEN %s AND %s\n", $1, $3, $5); $$ = true; }
    ;

value_expr:
    string_expr { $$ = $1; }
    | numeric_expr { 
        char *buf = malloc(32);
        sprintf(buf, "%.2f", $1);
        $$ = buf;
    }
    | boolean_expr { $$ = $1 ? strdup("true") : strdup("false"); }
    | column_name { $$ = $1; }
    | NULL { $$ = strdup("NULL"); }
    ;

string_expr:
    STRING { $$ = $1; }
    | QUOTED_STRING { $$ = $1; }
    ;

numeric_expr:
    FLOAT { $$ = $1; }
    | INTEGER { $$ = (double)$1; }
    | numeric_expr '+' numeric_expr { $$ = $1 + $3; }
    | numeric_expr '-' numeric_expr { $$ = $1 - $3; }
    | numeric_expr '*' numeric_expr { $$ = $1 * $3; }
    | numeric_expr '/' numeric_expr { 
        if ($3 == 0.0) {
            yyerror("Division by zero");
            $$ = 0.0;
        } else {
            $$ = $1 / $3;
        }
    }
    | numeric_expr '%' numeric_expr { $$ = fmod($1, $3); }
    | '-' numeric_expr %prec UMINUS { $$ = -$2; }
    | '(' numeric_expr ')' { $$ = $2; }
    ;

integer_expr:
    INTEGER { $$ = $1; }
    | integer_expr '+' integer_expr { $$ = $1 + $3; }
    | integer_expr '-' integer_expr { $$ = $1 - $3; }
    | integer_expr '*' integer_expr { $$ = $1 * $3; }
    | integer_expr '/' integer_expr { 
        if ($3 == 0) {
            yyerror("Division by zero");
            $$ = 0;
        } else {
            $$ = $1 / $3;
        }
    }
    | '(' integer_expr ')' { $$ = $2; }
    ;

boolean_expr:
    BOOLEAN { $$ = $1; }
    ;

table_name:
    IDENTIFIER { $$ = $1; }
    | MODELS { $$ = strdup("models"); }
    | AGENTS { $$ = strdup("agents"); }
    | STATUS { $$ = strdup("status"); }
    | METRICS { $$ = strdup("metrics"); }
    ;

column_name:
    IDENTIFIER { $$ = $1; }
    ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Query Parser Error: %s\n", s);
}

void init_query(void) {
    current_query.model_name = NULL;
    current_query.query_type = NULL;
    current_query.fields = NULL;
    current_query.field_count = 0;
    current_query.streaming = false;
    current_query.limit = -1;
}

void set_model(char *model) {
    if (selected_model) free(selected_model);
    selected_model = strdup(model);
    current_query.model_name = strdup(model);
}

void add_field(char *name, char *value) {
    current_query.field_count++;
    current_query.fields = realloc(current_query.fields, 
        sizeof(QueryField) * current_query.field_count);
    current_query.fields[current_query.field_count - 1].field_name = strdup(name);
    current_query.fields[current_query.field_count - 1].field_value = value ? strdup(value) : NULL;
}

void execute_query(void) {
    printf("Executing query with %d fields\n", current_query.field_count);
    if (current_query.model_name) {
        printf("Target model: %s\n", current_query.model_name);
    }
    if (current_query.limit > 0) {
        printf("Result limit: %d\n", current_query.limit);
    }
    print_results();
}

void print_results(void) {
    printf("Query Results:\n");
    for (int i = 0; i < current_query.field_count; i++) {
        printf("  Field: %s", current_query.fields[i].field_name);
        if (current_query.fields[i].field_value) {
            printf(" = %s", current_query.fields[i].field_value);
        }
        printf("\n");
    }
}

QueryResult process_inference_query(char *prompt, char *model) {
    QueryResult result;
    result.model_name = strdup(model);
    result.query_type = strdup("inference");
    result.fields = NULL;
    result.field_count = 0;
    result.streaming = false;
    result.limit = -1;
    
    add_field("prompt", prompt);
    return result;
}

QueryResult process_embedding_query(char *text, char *model) {
    QueryResult result;
    result.model_name = strdup(model);
    result.query_type = strdup("embedding");
    result.fields = NULL;
    result.field_count = 0;
    result.streaming = false;
    result.limit = -1;
    
    add_field("text", text);
    return result;
}

QueryResult process_status_query(char *agent_id) {
    QueryResult result;
    result.model_name = NULL;
    result.query_type = strdup("status");
    result.fields = NULL;
    result.field_count = 0;
    result.streaming = false;
    result.limit = -1;
    
    add_field("agent_id", agent_id);
    return result;
}

int main(void) {
    init_query();
    printf("Starting LLM Query Processor\n");
    return yyparse();
}