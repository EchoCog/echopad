grammar PromptTemplate;

// Grammar for advanced prompt template language with variables, conditionals, and loops

// Lexer rules
WS: [ \t\r\n]+ -> skip;
COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;

// Template delimiters
TEMPLATE_START: '{{';
TEMPLATE_END: '}}';
BLOCK_START: '{%';
BLOCK_END: '%}';

// Keywords
IF: 'if';
ELSE: 'else';
ELIF: 'elif';
ENDIF: 'endif';
FOR: 'for';
IN: 'in';
ENDFOR: 'endfor';
SET: 'set';
INCLUDE: 'include';
EXTENDS: 'extends';
BLOCK: 'block';
ENDBLOCK: 'endblock';
MACRO: 'macro';
ENDMACRO: 'endmacro';
CALL: 'call';
WITH: 'with';
CONTEXT: 'context';
ESCAPE: 'escape';
AUTOESCAPE: 'autoescape';
RAW: 'raw';
ENDRAW: 'endraw';

// Operators
AND: 'and';
OR: 'or';
NOT: 'not';
EQ: '==';
NE: '!=';
LT: '<';
LE: '<=';
GT: '>';
GE: '>=';
PLUS: '+';
MINUS: '-';
MULTIPLY: '*';
DIVIDE: '/';
MODULO: '%';
POWER: '**';
ASSIGN: '=';
DOT: '.';
PIPE: '|';

// Literals
STRING: '\'' (ESC | ~['\r\n])* '\'' | '"' (ESC | ~["\r\n])* '"';
fragment ESC: '\\' (['\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];

NUMBER: INT ('.' [0-9]+)?;
fragment INT: '-'? ('0' | [1-9] [0-9]*);

BOOLEAN: 'true' | 'false';
NULL: 'null';
NONE: 'none';

IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;

// Delimiters
LPAREN: '(';
RPAREN: ')';
LBRACKET: '[';
RBRACKET: ']';
COMMA: ',';
COLON: ':';
SEMICOLON: ';';

// Plain text (everything outside template expressions)
PLAIN_TEXT: ~[{]+;

// Parser rules
start: templateContent;

templateContent: (plainText | templateExpression | templateBlock)*;

plainText: PLAIN_TEXT;

templateExpression: TEMPLATE_START expression TEMPLATE_END;

templateBlock: 
    ifBlock
    | forBlock
    | setBlock
    | includeBlock
    | blockDef
    | macroDef
    | rawBlock
    ;

ifBlock: 
    BLOCK_START IF expression BLOCK_END templateContent
    (BLOCK_START ELIF expression BLOCK_END templateContent)*
    (BLOCK_START ELSE BLOCK_END templateContent)?
    BLOCK_START ENDIF BLOCK_END
    ;

forBlock:
    BLOCK_START FOR IDENTIFIER IN expression BLOCK_END
    templateContent
    BLOCK_START ENDFOR BLOCK_END
    ;

setBlock:
    BLOCK_START SET IDENTIFIER ASSIGN expression BLOCK_END
    ;

includeBlock:
    BLOCK_START INCLUDE STRING (WITH expression)? BLOCK_END
    ;

blockDef:
    BLOCK_START BLOCK IDENTIFIER BLOCK_END
    templateContent
    BLOCK_START ENDBLOCK BLOCK_END
    ;

macroDef:
    BLOCK_START MACRO IDENTIFIER LPAREN (parameterList)? RPAREN BLOCK_END
    templateContent
    BLOCK_START ENDMACRO BLOCK_END
    ;

rawBlock:
    BLOCK_START RAW BLOCK_END
    .*?
    BLOCK_START ENDRAW BLOCK_END
    ;

expression:
    orExpression
    ;

orExpression:
    andExpression (OR andExpression)*
    ;

andExpression:
    notExpression (AND notExpression)*
    ;

notExpression:
    NOT notExpression
    | comparisonExpression
    ;

comparisonExpression:
    arithmeticExpression ((EQ | NE | LT | LE | GT | GE) arithmeticExpression)*
    ;

arithmeticExpression:
    termExpression ((PLUS | MINUS) termExpression)*
    ;

termExpression:
    factorExpression ((MULTIPLY | DIVIDE | MODULO) factorExpression)*
    ;

factorExpression:
    powerExpression (POWER factorExpression)?
    ;

powerExpression:
    unaryExpression
    ;

unaryExpression:
    (PLUS | MINUS) unaryExpression
    | postfixExpression
    ;

postfixExpression:
    primaryExpression (
        DOT IDENTIFIER
        | LBRACKET expression RBRACKET
        | LPAREN argumentList? RPAREN
        | PIPE filterExpression
    )*
    ;

primaryExpression:
    IDENTIFIER
    | STRING
    | NUMBER
    | BOOLEAN
    | NULL
    | NONE
    | LPAREN expression RPAREN
    | listLiteral
    | dictLiteral
    ;

listLiteral:
    LBRACKET (expression (COMMA expression)*)? RBRACKET
    ;

dictLiteral:
    '{' (dictPair (COMMA dictPair)*)? '}'
    ;

dictPair:
    (IDENTIFIER | STRING) COLON expression
    ;

filterExpression:
    IDENTIFIER (COLON argumentList)?
    ;

argumentList:
    expression (COMMA expression)*
    ;

parameterList:
    IDENTIFIER (COMMA IDENTIFIER)*
    ;