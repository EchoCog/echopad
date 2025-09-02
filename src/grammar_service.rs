use crate::grammar_parser::{
    GrammarDefinition, GrammarType, create_parser, parse_grammar_file, ParseTree
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

        // Load a Z++ specification grammar
        let zpp_grammar = self.create_zpp_specification_grammar()?;
        self.add_grammar(zpp_grammar)?;
        
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