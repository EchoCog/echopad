# Comprehensive Grammar Definitions & Formal Specifications

This document describes the comprehensive grammar definitions and formal specifications implemented for the Paddler LLMOps platform using three major parser technologies: ANTLR, YACC/Bison, and Z++.

## Overview

The grammar system provides formal language definitions for:

- **LLM API Specifications**: Request/response formats, parameter validation
- **Configuration Languages**: Agent and balancer setup, model configuration  
- **Prompt Templates**: Dynamic template language with variables and control flow
- **Resource Management**: Allocation, optimization, and monitoring
- **Query Languages**: SQL-like interface for LLM operations
- **System Specifications**: Mathematical models of system behavior and properties

## ANTLR Grammars

ANTLR (ANother Tool for Language Recognition) grammars provide context-free grammar definitions that generate lexers and parsers.

### LLM API Grammar (`examples/grammars/llm_api.g4`)

Defines the complete grammar for LLM inference API requests and responses:

```antlr
grammar LLMApi;

start: apiRequest | apiResponse | toolDefinition | promptTemplate;

apiRequest: LBRACE requestFields RBRACE;
requestFields: requestField (COMMA requestField)*;
requestField: 
    MODEL COLON STRING
    | PROMPT COLON STRING
    | MESSAGES COLON messageArray
    | TEMPERATURE COLON NUMBER
    | MAX_TOKENS COLON NUMBER
    | STREAM COLON BOOLEAN
    | TOOLS COLON toolArray
    ;
```

**Features:**
- Complete request/response validation
- Function calling support with JSON schema
- Message array handling for chat completion
- Parameter validation with type checking
- Tool definitions for function calling

### Configuration Language Grammar (`examples/grammars/configuration.g4`)

Defines configuration syntax for agents, balancers, and models:

```antlr
grammar ConfigurationLanguage;

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
    | MAX_CONCURRENT EQUALS INTEGER SEMICOLON
    | HEALTH_CHECK LBRACE healthCheckSettings RBRACE
    ;
```

**Features:**
- Agent configuration with endpoints and capacity settings
- Load balancing strategy configuration
- Health check configuration
- Security and TLS settings
- Logging configuration

### Prompt Template Grammar (`examples/grammars/prompt_template.g4`)

Advanced template language with Jinja2-like syntax:

```antlr
grammar PromptTemplate;

templateBlock: 
    ifBlock
    | forBlock
    | setBlock
    | includeBlock
    ;

ifBlock: 
    BLOCK_START IF expression BLOCK_END templateContent
    (BLOCK_START ELIF expression BLOCK_END templateContent)*
    (BLOCK_START ELSE BLOCK_END templateContent)?
    BLOCK_START ENDIF BLOCK_END
    ;
```

**Features:**
- Variable interpolation with `{{ variable }}`
- Conditional blocks with `{% if condition %}`
- Loop constructs with `{% for item in list %}`
- Macro definitions and includes
- Filter expressions for data transformation

## YACC/Bison Grammars

YACC grammars provide bottom-up parsing with semantic actions for expression evaluation and command processing.

### Resource Management Grammar (`examples/grammars/resource_management.y`)

Comprehensive resource allocation and optimization language:

```yacc
%{
#include <stdio.h>
#include <math.h>
// Resource management structures
typedef struct {
    char *name;
    char *type;
    double value;
    int allocated;
} Resource;
%}

resource_statement:
    ALLOCATE IDENTIFIER NUMBER {
        allocate_resource($2, "generic", $3);
    }
    | OPTIMIZE resource_allocation_expr {
        printf("Optimizing resource allocation\n");
    }
    ;
```

**Features:**
- Resource allocation commands
- Agent registration and management
- Load balancing and optimization
- Mathematical expressions for resource calculations
- Constraint satisfaction for allocation problems

### LLM Query Language Grammar (`examples/grammars/llm_query.y`)

SQL-like query language for LLM operations:

```yacc
inference_query:
    INFERENCE FROM MODEL string_expr WITH inference_params {
        set_model($4);
        current_query = process_inference_query("default_prompt", $4);
    }
    | COMPLETION string_expr FROM MODEL string_expr {
        printf("Completion query: %s from model: %s\n", $2, $5);
    }
    ;
```

**Features:**
- SELECT queries for model and agent status
- INFERENCE commands for text generation
- EMBEDDING queries for vector generation
- Complex WHERE clauses with parameter filtering
- Aggregation functions (COUNT, AVG, MAX, MIN)

## Z++ Formal Specifications

Z++ provides mathematical specification language for formal system modeling.

### LLM System Specification (`examples/grammars/llm_system.zpp`)

Complete formal model of the LLM inference system:

```z
schema SystemState
  agents: AgentId ⤔ Agent
  models: ModelId ⤔ Model  
  activeRequests: ℙ SessionId
  pendingRequests: seq InferenceRequest
  totalTokensProcessed: ℕ
  systemUptime: ℕ

schema SystemInvariant
  SystemState
  ∀ a: ran agents • a.currentLoad ≤ a.maxConcurrent
  totalTokensProcessed ≥ 0
  #activeRequests ≤ (Σ a: ran agents • a.maxConcurrent)
```

**Features:**
- Complete system state modeling
- Safety and liveness properties
- Load balancing operations
- Resource management schemas
- Performance monitoring specifications

### Inference Workflow Specification (`examples/grammars/inference_workflow.zpp`)

Formal specification of the inference request processing workflow:

```z
schema InferenceWorkflowState
  pendingRequests: RequestId ⤔ InferenceWorkflowRequest
  activeRequests: RequestId ⤔ InferenceWorkflowRequest  
  completedRequests: RequestId ⤔ InferenceWorkflowResponse
  queueCapacity: ℕ
  activeCapacity: ℕ

schema SubmitRequest
  ΔInferenceWorkflowState
  newRequest?: InferenceWorkflowRequest
  result!: RequestSubmissionResult
  
  #pendingRequests < queueCapacity ⇒ (
    pendingRequests' = pendingRequests ∪ {newRequest?.id ↦ newRequest?} ∧
    result! = Accepted
  )
```

**Features:**
- Request lifecycle modeling
- Queue management specifications
- Streaming inference support
- Cache management operations
- Fault tolerance properties

## Code Generation

The grammar system supports code generation in multiple target languages:

### Rust Code Generation

```rust
// Generated parser for grammar: LLMOperations
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
    InferencerequestNode(InferencerequestNode),
    ParametersNode(ParametersNode),
}

pub struct LlmoperationsParser {
    grammar_name: String,
}
```

### TypeScript Code Generation

```typescript
// Generated TypeScript parser for grammar: LLMOperations
export interface AstNode {
  type: string;
  text?: string;
  children: AstNode[];
  parameters?: Record<string, any>;
  metadata?: Record<string, any>;
}

export class LlmoperationsParser {
  parse(input: string): AstNode { /* ... */ }
  validate(node: AstNode): boolean { /* ... */ }
}
```

### C Code Generation (YACC)

```c
/* Generated YACC parser for grammar: ResourceManagementSystem */
%{
#include <stdio.h>
#include <math.h>
#include <sys/resource.h>

typedef struct {
    char *name;
    double value;
    int allocated;
} Resource;
%}

%%
resource_allocation : 'allocate' IDENTIFIER NUMBER 'to' IDENTIFIER 
    { allocate_resource($2, $3, $5); }
    ;
%%
```

### LaTeX/Markdown Generation (Z++)

**LaTeX Output:**
```latex
\documentclass[11pt]{article}
\usepackage{oz}
\usepackage{zed-csp}

\begin{schema}{SystemState}
agents: AgentId \pfun Agent \\
models: ModelId \pfun Model \\
activeRequests: \power SessionId \\
\end{schema}
```

**Markdown Output:**
```markdown
# Z++ Specification: LLMSystemSpecification

## Schemas

### SystemState

```z
schema SystemState
  agents: AgentId ⤔ Agent
  models: ModelId ⤔ Model
  activeRequests: ℙ SessionId
```
```

## Usage Examples

### LLM API Request Parsing

```rust
use paddler::grammar_service::GrammarService;

let service = GrammarService::new();
service.load_default_grammars().unwrap();

let api_request = r#"{
    "model": "gpt-4",
    "messages": [
        {"role": "user", "content": "Hello world"}
    ],
    "temperature": 0.7,
    "max_tokens": 100
}"#;

let result = service.parse("LLMApiGrammar", api_request).unwrap();
```

### Resource Management Commands

```rust
let resource_commands = r#"
allocate cpu_cores 8 to worker_01;
register agent worker_01 with {cpu: 8, memory: 16384, gpu: 1};
optimize resources for worker_01, worker_02;
"#;

let result = service.parse("ResourceManagement", resource_commands).unwrap();
```

### Configuration Parsing

```rust
let config = r#"
agent worker1 {
    endpoint = "http://localhost:8080";
    max_concurrent = 10;
    cpu_threads = 4;
    gpu_layers = 32;
}

balancer main {
    strategy = "round_robin";
    health_check = {
        enabled = true;
        interval = 30;
    };
}
"#;

let result = service.parse("ConfigurationGrammar", config).unwrap();
```

## Mathematical Properties

The Z++ specifications prove several important properties:

### Safety Properties
- **Queue bounds**: `□(#pendingRequests ≤ queueCapacity)`
- **Load limits**: `□(∀ a: ran agents • a.currentLoad ≤ a.maxConcurrent)`
- **State consistency**: `□(dom activeRequests ∩ dom completedRequests = ∅)`

### Liveness Properties  
- **Request processing**: `□◇(pendingRequests = ⟨⟩)`
- **System progress**: `□◇(#completedRequests ≠ #completedRequests)`
- **Fault tolerance**: Requests eventually complete even with agent failures

### Correctness Properties
- **Request uniqueness**: Each request ID appears in at most one state collection
- **Resource conservation**: Total allocated resources never exceed capacity
- **Temporal ordering**: Responses maintain causal ordering with requests

## API Integration

The grammar service provides HTTP endpoints for dynamic grammar operations:

### Parse Endpoint
```http
POST /api/grammar/parse
Content-Type: application/json

{
  "grammar_name": "LLMApiGrammar", 
  "input": "{\"model\": \"gpt-4\", \"prompt\": \"Hello\"}"
}
```

### Code Generation Endpoint
```http
POST /api/grammar/generate
Content-Type: application/json

{
  "grammar_name": "ResourceManagement",
  "target_language": "rust"  
}
```

### Grammar Loading Endpoint
```http
POST /api/grammar/load
Content-Type: application/json

{
  "name": "CustomGrammar",
  "grammar_type": "antlr",
  "content": "grammar Custom; start: expr; expr: ID;"
}
```

## Testing

Comprehensive test suite covers:
- Grammar parsing and validation
- Code generation in all target languages
- Complex nested structures
- Error handling and recovery
- Performance with large inputs
- Integration with HTTP endpoints

Run tests with:
```bash
cargo test grammar_parser_comprehensive_tests
```

## Extensions

The grammar system is designed for extensibility:

1. **New Grammar Types**: Add support for PEG, EBNF, or custom formats
2. **Additional Target Languages**: Generate parsers for Python, Java, Go, etc.
3. **Domain-Specific Languages**: Create specialized grammars for AI/ML workflows
4. **Runtime Integration**: Connect with actual ANTLR, Yacc, and Z++ tools
5. **Optimization**: Add grammar analysis and optimization features

## Conclusion

This comprehensive grammar system provides formal language definitions across three major parser technologies, enabling precise specification and validation of LLMOps platform components. The combination of ANTLR for parsing, YACC for evaluation, and Z++ for mathematical modeling creates a robust foundation for system correctness and reliability.