use crate::grammar_parser::{
    GrammarDefinition, GrammarType, GrammarRule, create_parser, parse_grammar_file, ParseTree
};
use crate::service::Service;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use actix_web::{web, HttpResponse, Result as ActixResult};
use async_trait::async_trait;
use tokio::sync::broadcast;
use log::info;

/// Request to parse input using a specific grammar
#[derive(Debug, Deserialize)]
pub struct ParseRequest {
    pub grammar_name: String,
    pub input: String,
}

/// Response containing parse results
#[derive(Debug, Serialize)]
pub struct ParseResponse {
    pub success: bool,
    pub parse_tree: Option<ParseTree>,
    pub error: Option<String>,
}

/// Request to load a grammar from content
#[derive(Debug, Deserialize)]
pub struct LoadGrammarRequest {
    pub name: String,
    pub grammar_type: String,
    pub content: String,
}

/// Response to grammar loading
#[derive(Debug, Serialize)]
pub struct LoadGrammarResponse {
    pub success: bool,
    pub message: String,
}

/// Request to generate code from a grammar
#[derive(Debug, Deserialize)]
pub struct GenerateCodeRequest {
    pub grammar_name: String,
    pub target_language: String,
}

/// Response containing generated code
#[derive(Debug, Serialize)]
pub struct GenerateCodeResponse {
    pub success: bool,
    pub code: Option<String>,
    pub error: Option<String>,
}

/// Grammar parsing service that manages multiple grammars
pub struct GrammarService {
    grammars: Arc<RwLock<HashMap<String, GrammarDefinition>>>,
}

impl GrammarService {
    pub fn new() -> Self {
        Self {
            grammars: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load default grammars for common languages
    pub fn load_default_grammars(&self) -> Result<()> {
        info!("Loading default grammars");
        
        // Load a simple arithmetic grammar
        let arithmetic_grammar = self.create_arithmetic_grammar()?;
        self.add_grammar(arithmetic_grammar)?;
        
        // Load a JSON grammar
        let json_grammar = self.create_json_grammar()?;
        self.add_grammar(json_grammar)?;

        // Load comprehensive LLM API grammar
        let llm_api_grammar = self.create_llm_api_grammar()?;
        self.add_grammar(llm_api_grammar)?;

        // Load configuration language grammar
        let config_grammar = self.create_configuration_grammar()?;
        self.add_grammar(config_grammar)?;

        // Load prompt template grammar  
        let prompt_grammar = self.create_prompt_template_grammar()?;
        self.add_grammar(prompt_grammar)?;

        // Load resource management YACC grammar
        let resource_yacc_grammar = self.create_resource_management_yacc_grammar()?;
        self.add_grammar(resource_yacc_grammar)?;

        // Load LLM query YACC grammar
        let query_yacc_grammar = self.create_llm_query_yacc_grammar()?;
        self.add_grammar(query_yacc_grammar)?;

        // Load Z++ specification grammars
        let zpp_grammar = self.create_zpp_specification_grammar()?;
        self.add_grammar(zpp_grammar)?;

        let llm_system_zpp_grammar = self.create_llm_system_zpp_grammar()?;
        self.add_grammar(llm_system_zpp_grammar)?;

        let inference_workflow_zpp_grammar = self.create_inference_workflow_zpp_grammar()?;
        self.add_grammar(inference_workflow_zpp_grammar)?;
        
        Ok(())
    }

    fn create_arithmetic_grammar(&self) -> Result<GrammarDefinition> {
        let content = r#"
            grammar ArithmeticGrammar;
            start expr;
            expr: expr '+' term
            expr: expr '-' term
            expr: term
            term: term '*' factor
            term: term '/' factor
            term: factor
            factor: '(' expr ')'
            factor: NUMBER
        "#;
        
        parse_grammar_file(content, GrammarType::Antlr)
    }

    fn create_json_grammar(&self) -> Result<GrammarDefinition> {
        let content = r#"
            grammar JsonGrammar;
            start value;
            value: object
            value: array
            value: STRING
            value: NUMBER
            value: 'true'
            value: 'false'
            value: 'null'
            object: '{' '}'
            object: '{' members '}'
            members: pair
            members: members ',' pair
            pair: STRING ':' value
            array: '[' ']'
            array: '[' elements ']'
            elements: value
            elements: elements ',' value
        "#;
        
        parse_grammar_file(content, GrammarType::Antlr)
    }

    fn create_zpp_specification_grammar(&self) -> Result<GrammarDefinition> {
        use crate::grammar_parser::{GrammarRule, GrammarDefinition, GrammarType};

        Ok(GrammarDefinition {
            name: "ZPlusPlus".to_string(),
            grammar_type: GrammarType::ZPlusPlus,
            rules: vec![
                GrammarRule {
                    name: "System".to_string(),
                    production: "state: State; operations: Operations".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "State".to_string(),
                    production: "x: ℕ; y: ℕ; z: ℕ".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "Operations".to_string(),
                    production: "Init; Add; Subtract".to_string(),
                    action: None,
                },
            ],
            start_rule: "System".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn create_llm_api_grammar(&self) -> Result<GrammarDefinition> {
        let content = r#"
            grammar LLMApiGrammar;
            start apiRequest;
            apiRequest: '{' requestFields '}'
            requestFields: requestField (',' requestField)*
            requestField: 'model' ':' STRING
            requestField: 'prompt' ':' STRING  
            requestField: 'messages' ':' messageArray
            requestField: 'temperature' ':' NUMBER
            requestField: 'max_tokens' ':' NUMBER
            requestField: 'top_p' ':' NUMBER
            requestField: 'stream' ':' BOOLEAN
            requestField: 'tools' ':' toolArray
            messageArray: '[' (message (',' message)*)? ']'
            message: '{' messageFields '}'
            messageFields: messageField (',' messageField)*
            messageField: 'role' ':' STRING
            messageField: 'content' ':' STRING
            toolArray: '[' (toolDef (',' toolDef)*)? ']'
            toolDef: '{' toolFields '}'
            toolFields: toolField (',' toolField)*
            toolField: 'type' ':' 'function'
            toolField: 'function' ':' functionDef
            functionDef: '{' functionFields '}'
            functionFields: functionField (',' functionField)*
            functionField: 'name' ':' STRING
            functionField: 'description' ':' STRING
            functionField: 'parameters' ':' schemaDef
            schemaDef: '{' schemaFields '}'
            schemaFields: schemaField (',' schemaField)*
            schemaField: 'type' ':' STRING
            schemaField: 'properties' ':' propertiesDef
            propertiesDef: '{' (propertyDef (',' propertyDef)*)? '}'
            propertyDef: STRING ':' schemaDef
        "#;
        
        parse_grammar_file(content, GrammarType::Antlr)
    }

    fn create_configuration_grammar(&self) -> Result<GrammarDefinition> {
        let content = r#"
            grammar ConfigurationGrammar;
            start configuration;
            configuration: configItem+
            configItem: agentConfig
            configItem: balancerConfig
            configItem: modelConfig
            agentConfig: 'agent' IDENTIFIER '{' agentSettings '}'
            agentSettings: agentSetting+
            agentSetting: 'endpoint' '=' STRING ';'
            agentSetting: 'port' '=' INTEGER ';'
            agentSetting: 'max_concurrent' '=' INTEGER ';'
            agentSetting: 'slots' '=' INTEGER ';'
            agentSetting: 'cpu_threads' '=' INTEGER ';'
            agentSetting: 'gpu_layers' '=' INTEGER ';'
            balancerConfig: 'balancer' IDENTIFIER '{' balancerSettings '}'
            balancerSettings: balancerSetting+
            balancerSetting: 'port' '=' INTEGER ';'
            balancerSetting: 'strategy' '=' STRING ';'
            balancerSetting: 'health_check' '=' BOOLEAN ';'
            modelConfig: 'model' IDENTIFIER '{' modelSettings '}'
            modelSettings: modelSetting+
            modelSetting: 'path' '=' STRING ';'
            modelSetting: 'context_size' '=' INTEGER ';'
            modelSetting: 'temperature' '=' FLOAT ';'
        "#;
        
        parse_grammar_file(content, GrammarType::Antlr)
    }

    fn create_prompt_template_grammar(&self) -> Result<GrammarDefinition> {
        let content = r#"
            grammar PromptTemplateGrammar;
            start template;
            template: templateContent
            templateContent: (plainText | templateExpression | templateBlock)*
            plainText: PLAIN_TEXT
            templateExpression: '{{' expression '}}'
            templateBlock: ifBlock
            templateBlock: forBlock
            templateBlock: setBlock
            ifBlock: '{%' 'if' expression '%}' templateContent '{%' 'endif' '%}'
            forBlock: '{%' 'for' IDENTIFIER 'in' expression '%}' templateContent '{%' 'endfor' '%}'
            setBlock: '{%' 'set' IDENTIFIER '=' expression '%}'
            expression: orExpression
            orExpression: andExpression ('or' andExpression)*
            andExpression: comparisonExpression ('and' comparisonExpression)*
            comparisonExpression: primaryExpression (('==' | '!=' | '<' | '>' | '<=' | '>=') primaryExpression)?
            primaryExpression: IDENTIFIER
            primaryExpression: STRING
            primaryExpression: NUMBER
            primaryExpression: BOOLEAN
            primaryExpression: '(' expression ')'
        "#;
        
        parse_grammar_file(content, GrammarType::Antlr)
    }

    fn create_resource_management_yacc_grammar(&self) -> Result<GrammarDefinition> {
        Ok(GrammarDefinition {
            name: "ResourceManagement".to_string(),
            grammar_type: GrammarType::Yacc,
            rules: vec![
                GrammarRule {
                    name: "program".to_string(),
                    production: "statement_list".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "statement_list".to_string(),
                    production: "statement | statement_list ';' statement".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "resource_statement".to_string(),
                    production: "'allocate' IDENTIFIER NUMBER".to_string(),
                    action: Some("{ allocate_resource($2, $3); }".to_string()),
                },
                GrammarRule {
                    name: "agent_statement".to_string(),
                    production: "'register' 'agent' IDENTIFIER 'with' resource_spec".to_string(),
                    action: Some("{ register_agent($3, $5); }".to_string()),
                },
                GrammarRule {
                    name: "optimization_statement".to_string(),
                    production: "'optimize' resource_list 'for' agent_list".to_string(),
                    action: Some("{ optimize_allocation($2, $4); }".to_string()),
                },
                GrammarRule {
                    name: "resource_spec".to_string(),
                    production: "'cpu' ':' INTEGER | 'memory' ':' INTEGER | 'gpu' ':' INTEGER".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "expression".to_string(),
                    production: "expression '+' expression | expression '-' expression | expression '*' expression | expression '/' expression | NUMBER | IDENTIFIER | '(' expression ')'".to_string(),
                    action: Some("{ $$ = evaluate_expression($1, $2, $3); }".to_string()),
                },
            ],
            start_rule: "program".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn create_llm_query_yacc_grammar(&self) -> Result<GrammarDefinition> {
        Ok(GrammarDefinition {
            name: "LLMQueryLanguage".to_string(),
            grammar_type: GrammarType::Yacc,
            rules: vec![
                GrammarRule {
                    name: "query_statement".to_string(),
                    production: "select_statement | inference_query | status_query".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "select_statement".to_string(),
                    production: "'SELECT' select_list 'FROM' table_name where_clause".to_string(),
                    action: Some("{ execute_select($2, $4, $5); }".to_string()),
                },
                GrammarRule {
                    name: "inference_query".to_string(),
                    production: "'INFERENCE' STRING 'FROM' 'MODEL' STRING 'WITH' inference_params".to_string(),
                    action: Some("{ process_inference($2, $5, $7); }".to_string()),
                },
                GrammarRule {
                    name: "embedding_query".to_string(),
                    production: "'EMBEDDING' STRING 'FROM' 'MODEL' STRING".to_string(),
                    action: Some("{ process_embedding($2, $5); }".to_string()),
                },
                GrammarRule {
                    name: "status_query".to_string(),
                    production: "'SELECT' 'STATUS' 'FROM' 'AGENT' STRING".to_string(),
                    action: Some("{ get_agent_status($5); }".to_string()),
                },
                GrammarRule {
                    name: "inference_params".to_string(),
                    production: "inference_param | inference_params ',' inference_param".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "inference_param".to_string(),
                    production: "'temperature' '=' NUMBER | 'max_tokens' '=' INTEGER | 'top_p' '=' NUMBER | 'stream' '=' BOOLEAN".to_string(),
                    action: Some("{ set_parameter($1, $3); }".to_string()),
                },
            ],
            start_rule: "query_statement".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn create_llm_system_zpp_grammar(&self) -> Result<GrammarDefinition> {
        Ok(GrammarDefinition {
            name: "LLMSystemSpecification".to_string(),
            grammar_type: GrammarType::ZPlusPlus,
            rules: vec![
                GrammarRule {
                    name: "schema SystemState".to_string(),
                    production: "agents: AgentId ⤔ Agent; models: ModelId ⤔ Model; activeRequests: ℙ SessionId; pendingRequests: seq InferenceRequest; completedResponses: seq InferenceResponse; totalTokensProcessed: ℕ; systemUptime: ℕ".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema Agent".to_string(),
                    production: "id: AgentId; status: Status; model: ModelId; maxConcurrent: ℕ; currentLoad: ℕ; cpuCores: ℕ; memoryMB: ℕ; gpuLayers: ℕ; contextSize: ℕ".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema Model".to_string(),
                    production: "id: ModelId; name: 𝔽; path: 𝔽; size: ℕ; contextLength: ℕ; vocabulary: ℕ; status: Status; loadedOn: ℙ AgentId".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema SystemInvariant".to_string(),
                    production: "SystemState; ∀ a: ran agents • a.currentLoad ≤ a.maxConcurrent; totalTokensProcessed ≥ 0; systemUptime ≥ 0; #activeRequests ≤ (Σ a: ran agents • a.maxConcurrent)".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema ProcessRequest".to_string(),
                    production: "ΔSystemState; request?: InferenceRequest; selectedAgent!: AgentId; selectedAgent! ∈ dom agents; agents(selectedAgent!).status = Active; agents(selectedAgent!).currentLoad < agents(selectedAgent!).maxConcurrent; activeRequests' = activeRequests ∪ {request?.sessionId}".to_string(),
                    action: Some("Process an inference request by selecting an available agent".to_string()),
                },
                GrammarRule {
                    name: "schema CompleteRequest".to_string(),
                    production: "ΔSystemState; sessionId?: SessionId; response?: InferenceResponse; agent?: AgentId; sessionId? ∈ activeRequests; activeRequests' = activeRequests \\ {sessionId?}; agents'(agent?).currentLoad = agents(agent?).currentLoad - 1; completedResponses' = completedResponses ^ ⟨response?⟩".to_string(),
                    action: Some("Complete an inference request and update system state".to_string()),
                },
                GrammarRule {
                    name: "theorem SafetyProperty".to_string(),
                    production: "SystemSpec ⇒ □(∀ a: ran agents • a.currentLoad ≤ a.maxConcurrent)".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "theorem LivenessProperty".to_string(),
                    production: "SystemSpec ∧ ◇(∃ a: ran agents • a.status = Active) ⇒ □◇(pendingRequests = ⟨⟩)".to_string(),
                    action: None,
                },
            ],
            start_rule: "SystemState".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn create_inference_workflow_zpp_grammar(&self) -> Result<GrammarDefinition> {
        Ok(GrammarDefinition {
            name: "InferenceWorkflowSpecification".to_string(),
            grammar_type: GrammarType::ZPlusPlus,
            rules: vec![
                GrammarRule {
                    name: "schema InferenceWorkflowState".to_string(),
                    production: "pendingRequests: RequestId ⤔ InferenceWorkflowRequest; activeRequests: RequestId ⤔ InferenceWorkflowRequest; completedRequests: RequestId ⤔ InferenceWorkflowResponse; failedRequests: RequestId ⤔ (InferenceWorkflowRequest × 𝔽); tokenizationCache: Prompt ⤔ TokenizationResult; responseCache: (Prompt × InferenceParameters) ⤔ InferenceWorkflowResponse; queueCapacity: ℕ; activeCapacity: ℕ; currentTime: Timestamp".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema WorkflowInvariant".to_string(),
                    production: "InferenceWorkflowState; #pendingRequests ≤ queueCapacity; #activeRequests ≤ activeCapacity; dom pendingRequests ∩ dom activeRequests = ∅; dom activeRequests ∩ dom completedRequests = ∅; ∀ r: ran pendingRequests • r.state = Queued; ∀ r: ran activeRequests • r.state = Processing".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "schema SubmitRequest".to_string(),
                    production: "ΔInferenceWorkflowState; newRequest?: InferenceWorkflowRequest; result!: RequestSubmissionResult; newRequest?.id ∉ (dom pendingRequests ∪ dom activeRequests ∪ dom completedRequests); newRequest?.state = Queued; #pendingRequests < queueCapacity ⇒ (pendingRequests' = pendingRequests ∪ {newRequest?.id ↦ newRequest?} ∧ result! = Accepted)".to_string(),
                    action: Some("Submit a new inference request to the workflow queue".to_string()),
                },
                GrammarRule {
                    name: "schema ExecuteInference".to_string(),
                    production: "ΔInferenceWorkflowState; request?: InferenceWorkflowRequest; response!: InferenceWorkflowResponse; request?.id ∈ dom activeRequests; response!.requestId = request?.id; response!.processingDuration > 0; activeRequests' = {request?.id} ⩤ activeRequests; completedRequests' = completedRequests ∪ {request?.id ↦ response!}".to_string(),
                    action: Some("Execute inference for an active request and generate response".to_string()),
                },
                GrammarRule {
                    name: "schema StreamingInference".to_string(),
                    production: "ΔInferenceWorkflowState; request?: InferenceWorkflowRequest; chunks!: seq StreamingChunk; request?.id ∈ dom activeRequests; ∀ i: 1..#chunks! • chunks!(i).requestId = request?.id ∧ chunks!(i).chunkId = i; chunks!(#chunks!).finished = true".to_string(),
                    action: Some("Execute streaming inference with real-time token generation".to_string()),
                },
                GrammarRule {
                    name: "theorem QueueNeverOverflows".to_string(),
                    production: "WorkflowSpec ⇒ □(#pendingRequests ≤ queueCapacity)".to_string(),
                    action: None,
                },
                GrammarRule {
                    name: "theorem RequestsEventuallyProcessed".to_string(),
                    production: "WorkflowSpec ∧ □◇(#activeRequests < activeCapacity) ⇒ □(pendingRequests ≠ ∅ ⇒ ◇(#pendingRequests < #pendingRequests))".to_string(),
                    action: None,
                },
            ],
            start_rule: "InferenceWorkflowState".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Add a grammar to the service
    pub fn add_grammar(&self, grammar: GrammarDefinition) -> Result<()> {
        let name = grammar.name.clone();
        let mut grammars = self.grammars.write()
            .map_err(|_| anyhow!("Failed to acquire write lock on grammars"))?;
        grammars.insert(name, grammar);
        Ok(())
    }

    /// Parse input using a specific grammar
    pub fn parse(&self, grammar_name: &str, input: &str) -> Result<ParseTree> {
        let grammars = self.grammars.read()
            .map_err(|_| anyhow!("Failed to acquire read lock on grammars"))?;
        
        let grammar = grammars.get(grammar_name)
            .ok_or_else(|| anyhow!("Grammar '{}' not found", grammar_name))?;
        
        let parser = create_parser(grammar.clone());
        parser.parse(input)
    }

    /// Generate code from a grammar
    pub fn generate_code(&self, grammar_name: &str, target_language: &str) -> Result<String> {
        let grammars = self.grammars.read()
            .map_err(|_| anyhow!("Failed to acquire read lock on grammars"))?;
        
        let grammar = grammars.get(grammar_name)
            .ok_or_else(|| anyhow!("Grammar '{}' not found", grammar_name))?;
        
        let parser = create_parser(grammar.clone());
        parser.generate_code(grammar, target_language)
    }

    /// List available grammars
    pub fn list_grammars(&self) -> Result<Vec<String>> {
        let grammars = self.grammars.read()
            .map_err(|_| anyhow!("Failed to acquire read lock on grammars"))?;
        
        Ok(grammars.keys().cloned().collect())
    }
}

impl Default for GrammarService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Service for GrammarService {
    fn name(&self) -> &'static str {
        "grammar_service"
    }

    async fn run(&mut self, mut shutdown_rx: broadcast::Receiver<()>) -> Result<()> {
        info!("Starting Grammar Service");
        
        // Load default grammars on startup
        self.load_default_grammars()?;
        
        // Wait for shutdown signal
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("Grammar Service received shutdown signal");
            }
        }
        
        info!("Grammar Service stopped");
        Ok(())
    }
}

/// HTTP endpoint to parse input using a grammar
pub async fn parse_endpoint(
    service: web::Data<GrammarService>,
    request: web::Json<ParseRequest>,
) -> ActixResult<HttpResponse> {
    match service.parse(&request.grammar_name, &request.input) {
        Ok(parse_tree) => Ok(HttpResponse::Ok().json(ParseResponse {
            success: true,
            parse_tree: Some(parse_tree),
            error: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ParseResponse {
            success: false,
            parse_tree: None,
            error: Some(e.to_string()),
        })),
    }
}

/// HTTP endpoint to load a grammar
pub async fn load_grammar_endpoint(
    service: web::Data<GrammarService>,
    request: web::Json<LoadGrammarRequest>,
) -> ActixResult<HttpResponse> {
    let grammar_type = match request.grammar_type.as_str() {
        "antlr" => GrammarType::Antlr,
        "yacc" => GrammarType::Yacc,
        "z++" | "zpp" => GrammarType::ZPlusPlus,
        _ => {
            return Ok(HttpResponse::BadRequest().json(LoadGrammarResponse {
                success: false,
                message: format!("Unsupported grammar type: {}", request.grammar_type),
            }));
        }
    };

    match parse_grammar_file(&request.content, grammar_type) {
        Ok(mut grammar) => {
            grammar.name = request.name.clone();
            match service.add_grammar(grammar) {
                Ok(()) => Ok(HttpResponse::Ok().json(LoadGrammarResponse {
                    success: true,
                    message: format!("Grammar '{}' loaded successfully", request.name),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(LoadGrammarResponse {
                    success: false,
                    message: format!("Failed to load grammar: {}", e),
                })),
            }
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(LoadGrammarResponse {
            success: false,
            message: format!("Failed to parse grammar: {}", e),
        })),
    }
}

/// HTTP endpoint to generate code from a grammar
pub async fn generate_code_endpoint(
    service: web::Data<GrammarService>,
    request: web::Json<GenerateCodeRequest>,
) -> ActixResult<HttpResponse> {
    match service.generate_code(&request.grammar_name, &request.target_language) {
        Ok(code) => Ok(HttpResponse::Ok().json(GenerateCodeResponse {
            success: true,
            code: Some(code),
            error: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(GenerateCodeResponse {
            success: false,
            code: None,
            error: Some(e.to_string()),
        })),
    }
}

/// HTTP endpoint to list available grammars
pub async fn list_grammars_endpoint(
    service: web::Data<GrammarService>,
) -> ActixResult<HttpResponse> {
    match service.list_grammars() {
        Ok(grammars) => Ok(HttpResponse::Ok().json(grammars)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grammar_service_creation() {
        let service = GrammarService::new();
        assert!(service.load_default_grammars().is_ok());
        
        let grammars = service.list_grammars().unwrap();
        assert!(!grammars.is_empty());
        assert!(grammars.contains(&"ArithmeticGrammar".to_string()));
    }

    #[tokio::test]
    async fn test_parse_arithmetic() {
        let service = GrammarService::new();
        service.load_default_grammars().unwrap();
        
        let result = service.parse("ArithmeticGrammar", "2 + 3 * 4");
        assert!(result.is_ok());
        
        let parse_tree = result.unwrap();
        assert_eq!(parse_tree.node_type, "program");
    }

    #[tokio::test]
    async fn test_generate_code() {
        let service = GrammarService::new();
        service.load_default_grammars().unwrap();
        
        let result = service.generate_code("ArithmeticGrammar", "rust");
        assert!(result.is_ok());
        
        let code = result.unwrap();
        assert!(code.contains("Generated parser for grammar: ArithmeticGrammar"));
    }
}