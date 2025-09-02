%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Forward declarations
int yylex(void);
void yyerror(const char *s);

// Data structures for resource management
typedef struct {
    char *name;
    char *type;
    double value;
    int allocated;
} Resource;

typedef struct {
    char *agent_id;
    int cpu_cores;
    int memory_mb;
    int gpu_count;
    double load_factor;
} Agent;

// Global variables
Resource resources[1000];
Agent agents[100];
int resource_count = 0;
int agent_count = 0;

// Function prototypes
void allocate_resource(char *name, char *type, double amount);
void deallocate_resource(char *name);
double get_resource_usage(char *name);
void register_agent(char *id, int cpu, int memory, int gpu);
void update_agent_load(char *id, double load);
double calculate_optimal_allocation(char *agent_id, char *resource_type);
%}

// Token definitions
%union {
    double num;
    char *str;
    int ival;
}

%token <num> NUMBER
%token <str> IDENTIFIER STRING
%token <ival> INTEGER

// Keywords
%token ALLOCATE DEALLOCATE REGISTER UPDATE
%token AGENT RESOURCE CPU MEMORY GPU LOAD
%token IF THEN ELSE WHILE FOR
%token AND OR NOT
%token OPTIMIZE BALANCE SCALE
%token MAX MIN AVG SUM COUNT

// Operators
%token ASSIGN PLUS_ASSIGN MINUS_ASSIGN
%left OR
%left AND
%right NOT
%left EQ NE LT LE GT GE
%left '+' '-'
%left '*' '/' '%'
%right '^'
%right UMINUS

// Non-terminals
%type <num> expression
%type <str> resource_spec agent_spec
%type <ival> integer_expr

%%

program:
    /* empty */
    | program statement
    ;

statement:
    resource_statement ';'
    | agent_statement ';'
    | allocation_statement ';'
    | optimization_statement ';'
    | control_statement
    | expression_statement ';'
    ;

resource_statement:
    RESOURCE IDENTIFIER ':' resource_spec {
        printf("Defining resource %s with spec %s\n", $2, $4);
    }
    | ALLOCATE IDENTIFIER NUMBER {
        allocate_resource($2, "generic", $3);
        printf("Allocated %.2f units to resource %s\n", $3, $2);
    }
    | DEALLOCATE IDENTIFIER {
        deallocate_resource($2);
        printf("Deallocated resource %s\n", $2);
    }
    ;

agent_statement:
    REGISTER AGENT IDENTIFIER WITH agent_spec {
        printf("Registering agent %s with spec %s\n", $3, $5);
    }
    | UPDATE AGENT IDENTIFIER LOAD NUMBER {
        update_agent_load($3, $5);
        printf("Updated agent %s load to %.2f\n", $3, $5);
    }
    ;

allocation_statement:
    ALLOCATE expression TO IDENTIFIER {
        printf("Allocating %.2f units to %s\n", $2, $4);
    }
    | BALANCE RESOURCE IDENTIFIER ACROSS agent_list {
        printf("Balancing resource %s across agents\n", $3);
    }
    ;

optimization_statement:
    OPTIMIZE resource_allocation_expr {
        printf("Optimizing resource allocation\n");
    }
    | SCALE AGENT IDENTIFIER BY expression {
        printf("Scaling agent %s by factor %.2f\n", $3, $5);
    }
    ;

control_statement:
    IF '(' condition ')' THEN statement_block {
        printf("Conditional execution\n");
    }
    | IF '(' condition ')' THEN statement_block ELSE statement_block {
        printf("Conditional execution with else\n");
    }
    | WHILE '(' condition ')' statement_block {
        printf("Loop execution\n");
    }
    | FOR '(' IDENTIFIER '=' expression ';' condition ';' IDENTIFIER ASSIGN expression ')' statement_block {
        printf("For loop execution\n");
    }
    ;

statement_block:
    '{' program '}'
    ;

expression_statement:
    expression {
        printf("Expression result: %.2f\n", $1);
    }
    ;

expression:
    NUMBER { $$ = $1; }
    | IDENTIFIER {
        $$ = get_resource_usage($1);
    }
    | expression '+' expression { $$ = $1 + $3; }
    | expression '-' expression { $$ = $1 - $3; }
    | expression '*' expression { $$ = $1 * $3; }
    | expression '/' expression { 
        if ($3 == 0) {
            yyerror("Division by zero");
            $$ = 0;
        } else {
            $$ = $1 / $3;
        }
    }
    | expression '%' expression { $$ = fmod($1, $3); }
    | expression '^' expression { $$ = pow($1, $3); }
    | '-' expression %prec UMINUS { $$ = -$2; }
    | '(' expression ')' { $$ = $2; }
    | MAX '(' expression_list ')' {
        printf("Calculating maximum\n");
        $$ = $3; // Simplified - would implement proper max
    }
    | MIN '(' expression_list ')' {
        printf("Calculating minimum\n");
        $$ = $3; // Simplified - would implement proper min
    }
    | AVG '(' expression_list ')' {
        printf("Calculating average\n");
        $$ = $3; // Simplified - would implement proper average
    }
    | SUM '(' expression_list ')' {
        printf("Calculating sum\n");
        $$ = $3; // Simplified - would implement proper sum
    }
    | COUNT '(' agent_list ')' {
        printf("Counting agents\n");
        $$ = agent_count;
    }
    ;

expression_list:
    expression { $$ = $1; }
    | expression_list ',' expression { $$ = $1; } // Simplified
    ;

condition:
    expression EQ expression { $$ = ($1 == $3); }
    | expression NE expression { $$ = ($1 != $3); }
    | expression LT expression { $$ = ($1 < $3); }
    | expression LE expression { $$ = ($1 <= $3); }
    | expression GT expression { $$ = ($1 > $3); }
    | expression GE expression { $$ = ($1 >= $3); }
    | condition AND condition { $$ = $1 && $3; }
    | condition OR condition { $$ = $1 || $3; }
    | NOT condition { $$ = !$2; }
    | '(' condition ')' { $$ = $2; }
    ;

resource_spec:
    CPU ':' INTEGER { 
        char *spec = malloc(50);
        sprintf(spec, "cpu:%d", $3);
        $$ = spec;
    }
    | MEMORY ':' INTEGER {
        char *spec = malloc(50);
        sprintf(spec, "memory:%d", $3);
        $$ = spec;
    }
    | GPU ':' INTEGER {
        char *spec = malloc(50);
        sprintf(spec, "gpu:%d", $3);
        $$ = spec;
    }
    ;

agent_spec:
    '{' resource_spec_list '}' {
        $$ = strdup("agent_spec"); // Simplified
    }
    ;

resource_spec_list:
    resource_spec { $$ = $1; }
    | resource_spec_list ',' resource_spec { $$ = $1; } // Simplified
    ;

agent_list:
    IDENTIFIER { printf("Agent: %s\n", $1); }
    | agent_list ',' IDENTIFIER { printf("Agent: %s\n", $3); }
    ;

resource_allocation_expr:
    IDENTIFIER FOR agent_list {
        printf("Resource allocation expression for %s\n", $1);
    }
    | IDENTIFIER WITH constraint_list {
        printf("Resource allocation with constraints\n");
    }
    ;

constraint_list:
    constraint { }
    | constraint_list AND constraint { }
    ;

constraint:
    expression LT expression { printf("Constraint: less than\n"); }
    | expression GT expression { printf("Constraint: greater than\n"); }
    | expression EQ expression { printf("Constraint: equal to\n"); }
    ;

integer_expr:
    INTEGER { $$ = $1; }
    | integer_expr '+' integer_expr { $$ = $1 + $3; }
    | integer_expr '-' integer_expr { $$ = $1 - $3; }
    | integer_expr '*' integer_expr { $$ = $1 * $3; }
    | integer_expr '/' integer_expr { 
        if ($3 == 0) {
            yyerror("Division by zero in integer expression");
            $$ = 0;
        } else {
            $$ = $1 / $3;
        }
    }
    | '(' integer_expr ')' { $$ = $2; }
    ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Resource Management Parser Error: %s\n", s);
}

void allocate_resource(char *name, char *type, double amount) {
    if (resource_count < 1000) {
        resources[resource_count].name = strdup(name);
        resources[resource_count].type = strdup(type);
        resources[resource_count].value = amount;
        resources[resource_count].allocated = 1;
        resource_count++;
    }
}

void deallocate_resource(char *name) {
    for (int i = 0; i < resource_count; i++) {
        if (strcmp(resources[i].name, name) == 0) {
            resources[i].allocated = 0;
            break;
        }
    }
}

double get_resource_usage(char *name) {
    for (int i = 0; i < resource_count; i++) {
        if (strcmp(resources[i].name, name) == 0 && resources[i].allocated) {
            return resources[i].value;
        }
    }
    return 0.0;
}

void register_agent(char *id, int cpu, int memory, int gpu) {
    if (agent_count < 100) {
        agents[agent_count].agent_id = strdup(id);
        agents[agent_count].cpu_cores = cpu;
        agents[agent_count].memory_mb = memory;
        agents[agent_count].gpu_count = gpu;
        agents[agent_count].load_factor = 0.0;
        agent_count++;
    }
}

void update_agent_load(char *id, double load) {
    for (int i = 0; i < agent_count; i++) {
        if (strcmp(agents[i].agent_id, id) == 0) {
            agents[i].load_factor = load;
            break;
        }
    }
}

double calculate_optimal_allocation(char *agent_id, char *resource_type) {
    // Simplified optimization calculation
    return 1.0; // Would implement actual optimization logic
}

int main(void) {
    printf("Starting Resource Management System Parser\n");
    return yyparse();
}