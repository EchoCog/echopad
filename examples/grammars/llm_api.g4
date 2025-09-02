grammar LLMApi;

// Lexer rules
WS: [ \t\r\n]+ -> skip;
COMMENT: '//' ~[\r\n]* -> skip;
MULTILINE_COMMENT: '/*' .*? '*/' -> skip;

// Keywords
MODEL: 'model';
PROMPT: 'prompt';
TEMPERATURE: 'temperature';
MAX_TOKENS: 'max_tokens';
TOP_P: 'top_p';
TOP_K: 'top_k';
FREQUENCY_PENALTY: 'frequency_penalty';
PRESENCE_PENALTY: 'presence_penalty';
STOP: 'stop';
STREAM: 'stream';
TOOLS: 'tools';
FUNCTION: 'function';
PARAMETERS: 'parameters';
PROPERTIES: 'properties';
REQUIRED: 'required';
TYPE: 'type';
DESCRIPTION: 'description';
ENUM: 'enum';
MESSAGES: 'messages';
ROLE: 'role';
CONTENT: 'content';
SYSTEM: 'system';
USER: 'user';
ASSISTANT: 'assistant';
TOOL: 'tool';

// Literals
STRING: '"' (ESC | ~["\r\n])* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];

NUMBER: INT ('.' [0-9]+)? EXP?;
fragment INT: '-'? ('0' | [1-9] [0-9]*);
fragment EXP: [Ee] [+\-]? INT;

BOOLEAN: 'true' | 'false';
NULL: 'null';

IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;

// Operators and delimiters
LBRACE: '{';
RBRACE: '}';
LBRACKET: '[';
RBRACKET: ']';
LPAREN: '(';
RPAREN: ')';
COMMA: ',';
COLON: ':';
SEMICOLON: ';';
DOT: '.';

// Parser rules
start: apiRequest | apiResponse | toolDefinition | promptTemplate;

apiRequest: LBRACE requestFields RBRACE;
requestFields: requestField (COMMA requestField)*;
requestField: 
    MODEL COLON STRING
    | PROMPT COLON STRING
    | MESSAGES COLON messageArray
    | TEMPERATURE COLON NUMBER
    | MAX_TOKENS COLON NUMBER
    | TOP_P COLON NUMBER
    | TOP_K COLON NUMBER
    | FREQUENCY_PENALTY COLON NUMBER
    | PRESENCE_PENALTY COLON NUMBER
    | STOP COLON (STRING | stringArray)
    | STREAM COLON BOOLEAN
    | TOOLS COLON toolArray
    ;

messageArray: LBRACKET (message (COMMA message)*)? RBRACKET;
message: LBRACE messageFields RBRACE;
messageFields: messageField (COMMA messageField)*;
messageField:
    ROLE COLON (SYSTEM | USER | ASSISTANT | TOOL)
    | CONTENT COLON STRING
    ;

stringArray: LBRACKET (STRING (COMMA STRING)*)? RBRACKET;

toolArray: LBRACKET (toolDefinition (COMMA toolDefinition)*)? RBRACKET;

toolDefinition: LBRACE toolFields RBRACE;
toolFields: toolField (COMMA toolField)*;
toolField:
    TYPE COLON FUNCTION
    | FUNCTION COLON functionDefinition
    ;

functionDefinition: LBRACE functionFields RBRACE;
functionFields: functionField (COMMA functionField)*;
functionField:
    IDENTIFIER COLON STRING
    | DESCRIPTION COLON STRING
    | PARAMETERS COLON schemaDefinition
    ;

schemaDefinition: LBRACE schemaFields RBRACE;
schemaFields: schemaField (COMMA schemaField)*;
schemaField:
    TYPE COLON STRING
    | PROPERTIES COLON propertiesDefinition
    | REQUIRED COLON stringArray
    | ENUM COLON stringArray
    ;

propertiesDefinition: LBRACE propertyDefinitions RBRACE;
propertyDefinitions: propertyDefinition (COMMA propertyDefinition)*;
propertyDefinition: STRING COLON schemaDefinition;

apiResponse: LBRACE responseFields RBRACE;
responseFields: responseField (COMMA responseField)*;
responseField:
    STRING COLON (STRING | NUMBER | BOOLEAN | NULL | object | array)
    ;

object: LBRACE (objectField (COMMA objectField)*)? RBRACE;
objectField: STRING COLON value;

array: LBRACKET (value (COMMA value)*)? RBRACKET;

value: STRING | NUMBER | BOOLEAN | NULL | object | array;

promptTemplate: LBRACE promptFields RBRACE;
promptFields: promptField (COMMA promptField)*;
promptField:
    IDENTIFIER COLON STRING
    | 'template' COLON STRING
    | 'variables' COLON stringArray
    | 'metadata' COLON object
    ;