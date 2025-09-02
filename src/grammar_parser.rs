use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

/// Grammar rule definition for parser generators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarRule {
    pub name: String,
    pub production: String,
    pub action: Option<String>,
}

/// Grammar definition supporting ANTLR, YACC, and Z++ styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarDefinition {
    pub name: String,
    pub grammar_type: GrammarType,
    pub rules: Vec<GrammarRule>,
    pub start_rule: String,
    pub metadata: HashMap<String, String>,
}

/// Supported grammar types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrammarType {
    Antlr,
    Yacc,
    ZPlusPlus,
}

/// Parser interface for different grammar types
pub trait GrammarParser {
    fn parse(&self, input: &str) -> Result<ParseTree>;
    fn validate_grammar(&self, grammar: &GrammarDefinition) -> Result<()>;
    fn generate_code(&self, grammar: &GrammarDefinition, language: &str) -> Result<String>;
}

/// Parse tree representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseTree {
    pub node_type: String,
    pub value: Option<String>,
    pub children: Vec<ParseTree>,
    pub span: Option<(usize, usize)>,
}

/// ANTLR-style grammar parser implementation
pub struct AntlrParser {
    grammar: GrammarDefinition,
}

impl AntlrParser {
    pub fn new(grammar: GrammarDefinition) -> Self {
        Self { grammar }
    }
}

impl GrammarParser for AntlrParser {
    fn parse(&self, input: &str) -> Result<ParseTree> {
        // Basic implementation - would integrate with actual ANTLR runtime in production
        Ok(ParseTree {
            node_type: "program".to_string(),
            value: Some(input.to_string()),
            children: vec![],
            span: Some((0, input.len())),
        })
    }

    fn validate_grammar(&self, grammar: &GrammarDefinition) -> Result<()> {
        if grammar.rules.is_empty() {
            return Err(anyhow!("Grammar must have at least one rule"));
        }
        
        if !grammar.rules.iter().any(|rule| rule.name == grammar.start_rule) {
            return Err(anyhow!("Start rule '{}' not found in grammar rules", grammar.start_rule));
        }
        
        Ok(())
    }

    fn generate_code(&self, grammar: &GrammarDefinition, language: &str) -> Result<String> {
        match language {
            "rust" => self.generate_rust_code(grammar),
            "typescript" => self.generate_typescript_code(grammar),
            _ => Err(anyhow!("Unsupported target language: {}", language)),
        }
    }
}

impl AntlrParser {
    fn generate_rust_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("// Generated parser for grammar: {}\n\n", grammar.name));
        code.push_str("use serde::{Deserialize, Serialize};\n\n");
        
        for rule in &grammar.rules {
            code.push_str(&format!("// Rule: {}\n", rule.name));
            code.push_str(&format!("// Production: {}\n", rule.production));
            if let Some(action) = &rule.action {
                code.push_str(&format!("// Action: {}\n", action));
            }
            code.push('\n');
        }
        
        Ok(code)
    }

    fn generate_typescript_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("// Generated parser for grammar: {}\n\n", grammar.name));
        
        for rule in &grammar.rules {
            code.push_str(&format!("// Rule: {}\n", rule.name));
            code.push_str(&format!("// Production: {}\n", rule.production));
            if let Some(action) = &rule.action {
                code.push_str(&format!("// Action: {}\n", action));
            }
            code.push('\n');
        }
        
        Ok(code)
    }
}

/// YACC-style grammar parser implementation
pub struct YaccParser {
    grammar: GrammarDefinition,
}

impl YaccParser {
    pub fn new(grammar: GrammarDefinition) -> Self {
        Self { grammar }
    }
}

impl GrammarParser for YaccParser {
    fn parse(&self, input: &str) -> Result<ParseTree> {
        // Basic implementation - would integrate with actual YACC/Bison runtime
        Ok(ParseTree {
            node_type: "yacc_program".to_string(),
            value: Some(input.to_string()),
            children: vec![],
            span: Some((0, input.len())),
        })
    }

    fn validate_grammar(&self, grammar: &GrammarDefinition) -> Result<()> {
        if grammar.rules.is_empty() {
            return Err(anyhow!("YACC grammar must have at least one rule"));
        }
        Ok(())
    }

    fn generate_code(&self, grammar: &GrammarDefinition, language: &str) -> Result<String> {
        match language {
            "c" => self.generate_c_code(grammar),
            "rust" => self.generate_rust_code(grammar),
            _ => Err(anyhow!("Unsupported target language for YACC: {}", language)),
        }
    }
}

impl YaccParser {
    fn generate_c_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("/* Generated YACC parser for grammar: {} */\n\n", grammar.name));
        code.push_str("%{\n#include <stdio.h>\n%}\n\n");
        
        code.push_str("%%\n");
        for rule in &grammar.rules {
            code.push_str(&format!("{} : {} ", rule.name, rule.production));
            if let Some(action) = &rule.action {
                code.push_str(&format!("{{ {} }}", action));
            }
            code.push_str(";\n");
        }
        code.push_str("%%\n");
        
        Ok(code)
    }

    fn generate_rust_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("// Generated YACC-style parser for grammar: {}\n\n", grammar.name));
        
        for rule in &grammar.rules {
            code.push_str(&format!("// YACC Rule: {} : {}\n", rule.name, rule.production));
        }
        
        Ok(code)
    }
}

/// Z++ formal specification parser
pub struct ZPlusPlusParser {
    grammar: GrammarDefinition,
}

impl ZPlusPlusParser {
    pub fn new(grammar: GrammarDefinition) -> Self {
        Self { grammar }
    }
}

impl GrammarParser for ZPlusPlusParser {
    fn parse(&self, input: &str) -> Result<ParseTree> {
        // Basic implementation for Z++ formal specifications
        Ok(ParseTree {
            node_type: "z_specification".to_string(),
            value: Some(input.to_string()),
            children: vec![],
            span: Some((0, input.len())),
        })
    }

    fn validate_grammar(&self, grammar: &GrammarDefinition) -> Result<()> {
        if grammar.rules.is_empty() {
            return Err(anyhow!("Z++ specification must have at least one schema"));
        }
        Ok(())
    }

    fn generate_code(&self, grammar: &GrammarDefinition, language: &str) -> Result<String> {
        match language {
            "latex" => self.generate_latex_code(grammar),
            "markdown" => self.generate_markdown_code(grammar),
            _ => Err(anyhow!("Unsupported target language for Z++: {}", language)),
        }
    }
}

impl ZPlusPlusParser {
    fn generate_latex_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("% Generated Z++ specification: {}\n\n", grammar.name));
        code.push_str("\\documentclass{article}\n");
        code.push_str("\\usepackage{zed-csp}\n");
        code.push_str("\\begin{document}\n\n");
        
        for rule in &grammar.rules {
            code.push_str(&format!("\\begin{{schema}}{{{}}}\n", rule.name));
            code.push_str(&format!("{}\n", rule.production));
            code.push_str("\\end{schema}\n\n");
        }
        
        code.push_str("\\end{document}\n");
        Ok(code)
    }

    fn generate_markdown_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("# Z++ Specification: {}\n\n", grammar.name));
        
        for rule in &grammar.rules {
            code.push_str(&format!("## Schema: {}\n\n", rule.name));
            code.push_str(&format!("```\n{}\n```\n\n", rule.production));
        }
        
        Ok(code)
    }
}

/// Factory for creating grammar parsers
pub fn create_parser(grammar: GrammarDefinition) -> Box<dyn GrammarParser> {
    match grammar.grammar_type {
        GrammarType::Antlr => Box::new(AntlrParser::new(grammar)),
        GrammarType::Yacc => Box::new(YaccParser::new(grammar)),
        GrammarType::ZPlusPlus => Box::new(ZPlusPlusParser::new(grammar)),
    }
}

/// Utility function to parse grammar file content
pub fn parse_grammar_file(content: &str, grammar_type: GrammarType) -> Result<GrammarDefinition> {
    // Basic parser for grammar files - would be more sophisticated in production
    let lines: Vec<&str> = content.lines().collect();
    let mut rules = Vec::new();
    let mut name = "unnamed_grammar".to_string();
    let mut start_rule = "start".to_string();
    
    for line in lines {
        let line = line.trim();
        if line.starts_with("grammar ") {
            name = line.strip_prefix("grammar ").unwrap().trim_end_matches(';').to_string();
        } else if line.starts_with("start ") {
            start_rule = line.strip_prefix("start ").unwrap().trim_end_matches(';').to_string();
        } else if line.contains(':') && !line.starts_with("//") && !line.starts_with("#") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                rules.push(GrammarRule {
                    name: parts[0].trim().to_string(),
                    production: parts[1].trim().trim_end_matches(';').to_string(),
                    action: None,
                });
            }
        }
    }
    
    if rules.is_empty() {
        return Err(anyhow!("No rules found in grammar file"));
    }
    
    Ok(GrammarDefinition {
        name,
        grammar_type,
        rules,
        start_rule,
        metadata: HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antlr_parser_creation() {
        let grammar = GrammarDefinition {
            name: "test_grammar".to_string(),
            grammar_type: GrammarType::Antlr,
            rules: vec![GrammarRule {
                name: "start".to_string(),
                production: "ID".to_string(),
                action: None,
            }],
            start_rule: "start".to_string(),
            metadata: HashMap::new(),
        };
        
        let parser = create_parser(grammar);
        assert!(parser.parse("test").is_ok());
    }

    #[test]
    fn test_yacc_parser_creation() {
        let grammar = GrammarDefinition {
            name: "test_yacc".to_string(),
            grammar_type: GrammarType::Yacc,
            rules: vec![GrammarRule {
                name: "expr".to_string(),
                production: "ID '+' ID".to_string(),
                action: Some("$$ = $1 + $3".to_string()),
            }],
            start_rule: "expr".to_string(),
            metadata: HashMap::new(),
        };
        
        let parser = create_parser(grammar);
        assert!(parser.parse("a + b").is_ok());
    }

    #[test]
    fn test_zpp_parser_creation() {
        let grammar = GrammarDefinition {
            name: "test_zpp".to_string(),
            grammar_type: GrammarType::ZPlusPlus,
            rules: vec![GrammarRule {
                name: "State".to_string(),
                production: "x: ℕ; y: ℕ".to_string(),
                action: None,
            }],
            start_rule: "State".to_string(),
            metadata: HashMap::new(),
        };
        
        let parser = create_parser(grammar);
        assert!(parser.parse("x = 5; y = 10").is_ok());
    }

    #[test]
    fn test_parse_grammar_file() {
        let content = r#"
            grammar TestGrammar;
            start expr;
            expr: ID '+' ID
            term: ID
        "#;
        
        let result = parse_grammar_file(content, GrammarType::Antlr);
        assert!(result.is_ok());
        
        let grammar = result.unwrap();
        assert_eq!(grammar.name, "TestGrammar");
        assert_eq!(grammar.start_rule, "expr");
        assert_eq!(grammar.rules.len(), 2);
    }
}