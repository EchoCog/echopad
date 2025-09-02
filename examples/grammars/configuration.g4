grammar ConfigurationLanguage;

// Configuration language for LLM agent and balancer setup

// Lexer rules
WS: [ \t\r\n]+ -> skip;
COMMENT: '#' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;

// Keywords
AGENT: 'agent';
BALANCER: 'balancer';
MODEL: 'model';
ENDPOINT: 'endpoint';
PORT: 'port';
HOST: 'host';
TIMEOUT: 'timeout';
RETRIES: 'retries';
MAX_CONCURRENT: 'max_concurrent';
LOAD_BALANCING: 'load_balancing';
HEALTH_CHECK: 'health_check';
LOGGING: 'logging';
METRICS: 'metrics';
SECURITY: 'security';
AUTHENTICATION: 'authentication';
AUTHORIZATION: 'authorization';
TLS: 'tls';
ENABLED: 'enabled';
DISABLED: 'disabled';
ROUND_ROBIN: 'round_robin';
LEAST_CONNECTIONS: 'least_connections';
WEIGHTED: 'weighted';
STICKY: 'sticky';
STRATEGY: 'strategy';
INTERVAL: 'interval';
PATH: 'path';
METHOD: 'method';
HEADERS: 'headers';
SLOTS: 'slots';
CPU_THREADS: 'cpu_threads';
GPU_LAYERS: 'gpu_layers';
CONTEXT_SIZE: 'context_size';
BATCH_SIZE: 'batch_size';
TEMPERATURE: 'temperature';
TOP_P: 'top_p';
TOP_K: 'top_k';
REPEAT_PENALTY: 'repeat_penalty';
SEED: 'seed';

// Types
STRING: '"' (ESC | ~["\r\n])* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];

INTEGER: [0-9]+;
FLOAT: [0-9]+ '.' [0-9]+;
BOOLEAN: 'true' | 'false';

IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
IPV4: [0-9]{1,3} '.' [0-9]{1,3} '.' [0-9]{1,3} '.' [0-9]{1,3};
URL: 'http' 's'? '://' ~[ \t\r\n;{}]+;

// Delimiters
LBRACE: '{';
RBRACE: '}';
LBRACKET: '[';
RBRACKET: ']';
SEMICOLON: ';';
COLON: ':';
COMMA: ',';
EQUALS: '=';

// Parser rules
start: configuration;

configuration: configurationItem+;

configurationItem:
    agentConfig
    | balancerConfig
    | modelConfig
    | loggingConfig
    | securityConfig
    ;

agentConfig: AGENT IDENTIFIER LBRACE agentSettings RBRACE;
agentSettings: agentSetting+;
agentSetting:
    ENDPOINT EQUALS (STRING | URL) SEMICOLON
    | PORT EQUALS INTEGER SEMICOLON
    | HOST EQUALS (STRING | IPV4) SEMICOLON
    | MAX_CONCURRENT EQUALS INTEGER SEMICOLON
    | SLOTS EQUALS INTEGER SEMICOLON
    | CPU_THREADS EQUALS INTEGER SEMICOLON
    | GPU_LAYERS EQUALS INTEGER SEMICOLON
    | CONTEXT_SIZE EQUALS INTEGER SEMICOLON
    | BATCH_SIZE EQUALS INTEGER SEMICOLON
    | HEALTH_CHECK LBRACE healthCheckSettings RBRACE
    | MODEL EQUALS STRING SEMICOLON
    | TIMEOUT EQUALS INTEGER SEMICOLON
    | RETRIES EQUALS INTEGER SEMICOLON
    ;

balancerConfig: BALANCER IDENTIFIER LBRACE balancerSettings RBRACE;
balancerSettings: balancerSetting+;
balancerSetting:
    PORT EQUALS INTEGER SEMICOLON
    | HOST EQUALS (STRING | IPV4) SEMICOLON
    | LOAD_BALANCING LBRACE loadBalancingSettings RBRACE
    | HEALTH_CHECK LBRACE healthCheckSettings RBRACE
    | TIMEOUT EQUALS INTEGER SEMICOLON
    | RETRIES EQUALS INTEGER SEMICOLON
    ;

modelConfig: MODEL IDENTIFIER LBRACE modelSettings RBRACE;
modelSettings: modelSetting+;
modelSetting:
    PATH EQUALS STRING SEMICOLON
    | CONTEXT_SIZE EQUALS INTEGER SEMICOLON
    | BATCH_SIZE EQUALS INTEGER SEMICOLON
    | TEMPERATURE EQUALS FLOAT SEMICOLON
    | TOP_P EQUALS FLOAT SEMICOLON
    | TOP_K EQUALS INTEGER SEMICOLON
    | REPEAT_PENALTY EQUALS FLOAT SEMICOLON
    | SEED EQUALS INTEGER SEMICOLON
    | CPU_THREADS EQUALS INTEGER SEMICOLON
    | GPU_LAYERS EQUALS INTEGER SEMICOLON
    ;

loadBalancingSettings: loadBalancingSetting+;
loadBalancingSetting:
    STRATEGY EQUALS (ROUND_ROBIN | LEAST_CONNECTIONS | WEIGHTED | STICKY) SEMICOLON
    | ENABLED EQUALS BOOLEAN SEMICOLON
    ;

healthCheckSettings: healthCheckSetting+;
healthCheckSetting:
    ENABLED EQUALS BOOLEAN SEMICOLON
    | INTERVAL EQUALS INTEGER SEMICOLON
    | PATH EQUALS STRING SEMICOLON
    | METHOD EQUALS STRING SEMICOLON
    | TIMEOUT EQUALS INTEGER SEMICOLON
    ;

loggingConfig: LOGGING LBRACE loggingSettings RBRACE;
loggingSettings: loggingSetting+;
loggingSetting:
    ENABLED EQUALS BOOLEAN SEMICOLON
    | PATH EQUALS STRING SEMICOLON
    | IDENTIFIER EQUALS (STRING | INTEGER | BOOLEAN) SEMICOLON
    ;

securityConfig: SECURITY LBRACE securitySettings RBRACE;
securitySettings: securitySetting+;
securitySetting:
    TLS LBRACE tlsSettings RBRACE
    | AUTHENTICATION LBRACE authSettings RBRACE
    | AUTHORIZATION LBRACE authzSettings RBRACE
    ;

tlsSettings: tlsSetting+;
tlsSetting:
    ENABLED EQUALS BOOLEAN SEMICOLON
    | IDENTIFIER EQUALS STRING SEMICOLON
    ;

authSettings: authSetting+;
authSetting:
    ENABLED EQUALS BOOLEAN SEMICOLON
    | IDENTIFIER EQUALS (STRING | BOOLEAN) SEMICOLON
    ;

authzSettings: authzSetting+;
authzSetting:
    ENABLED EQUALS BOOLEAN SEMICOLON
    | IDENTIFIER EQUALS (STRING | BOOLEAN | LBRACKET stringList RBRACKET) SEMICOLON
    ;

stringList: STRING (COMMA STRING)*;