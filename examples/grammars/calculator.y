%{
#include <stdio.h>
#include <stdlib.h>
int yylex(void);
void yyerror(const char *s);
%}

%token NUMBER IDENTIFIER
%left '+' '-'
%left '*' '/'
%right UMINUS

%%

expression: 
    expression '+' expression { $$ = $1 + $3; }
  | expression '-' expression { $$ = $1 - $3; }  
  | expression '*' expression { $$ = $1 * $3; }
  | expression '/' expression { $$ = $1 / $3; }
  | '-' expression %prec UMINUS { $$ = -$2; }
  | '(' expression ')' { $$ = $2; }
  | NUMBER { $$ = $1; }
  | IDENTIFIER { $$ = get_variable($1); }
  ;

%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}

int main(void) {
    return yyparse();
}