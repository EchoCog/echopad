grammar ArithmeticCalculator;
start expression;

expression: expression '+' term
expression: expression '-' term
expression: term

term: term '*' factor  
term: term '/' factor
term: factor

factor: '(' expression ')'
factor: NUMBER
factor: IDENTIFIER