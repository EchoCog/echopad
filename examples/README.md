# Grammar Examples for EchoCog

This directory contains example grammar files for different parser generators supported by EchoCog.

## Supported Grammar Types

- **ANTLR**: Context-free grammars for generating lexers and parsers
- **YACC**: Yacc/Bison compatible grammar definitions  
- **Z++**: Formal specification language schemas

## Examples

### ANTLR Grammar (Simple Calculator)
```antlr
grammar Calculator;
start expr;
expr: expr '+' term
expr: expr '-' term  
expr: term
term: term '*' factor
term: term '/' factor
term: factor
factor: '(' expr ')'
factor: NUMBER
```

### YACC Grammar (Expression Parser)
```yacc
%{
#include <stdio.h>
%}

%%
expression : expression '+' term   { $$ = $1 + $3; }
           | expression '-' term   { $$ = $1 - $3; }
           | term                  { $$ = $1; }
           ;
term       : term '*' factor       { $$ = $1 * $3; }
           | term '/' factor       { $$ = $1 / $3; }
           | factor                { $$ = $1; }
           ;
factor     : '(' expression ')'    { $$ = $2; }
           | NUMBER                { $$ = $1; }
           ;
%%
```

### Z++ Specification (State Machine)
```z++
schema StateMachine
state: State
current: State
operations: Operations

schema State
position: ℕ
velocity: ℕ
active: Boolean

schema Operations
Init
  state' = {position ↦ 0, velocity ↦ 0, active ↦ true}

Move
  Δ(state)
  state'.position = state.position + state.velocity
  state'.velocity = state.velocity
  state'.active = state.active

Stop
  Δ(state)
  state'.position = state.position  
  state'.velocity = 0
  state'.active = false
```

## API Usage

You can use these grammars via the REST API:

### Load a Grammar
```bash
curl -X POST http://localhost:8080/grammar/load \
  -H "Content-Type: application/json" \
  -d '{
    "name": "calculator",
    "grammar_type": "antlr", 
    "content": "grammar Calculator;\nstart expr;\nexpr: NUMBER;"
  }'
```

### Parse Input
```bash
curl -X POST http://localhost:8080/grammar/parse \
  -H "Content-Type: application/json" \
  -d '{
    "grammar_name": "calculator",
    "input": "42 + 17"
  }'
```

### Generate Code  
```bash
curl -X POST http://localhost:8080/grammar/generate \
  -H "Content-Type: application/json" \
  -d '{
    "grammar_name": "calculator",
    "target_language": "rust"
  }'
```

### List Available Grammars
```bash
curl http://localhost:8080/grammar/list
```