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
        code.push_str("use serde::{Deserialize, Serialize};\n");
        code.push_str("use std::collections::HashMap;\n\n");
        
        // Generate AST node structures
        code.push_str("// AST Node definitions\n");
        code.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
        code.push_str("pub enum AstNode {\n");
        
        for rule in &grammar.rules {
            let node_name = self.capitalize_rule_name(&rule.name);
            code.push_str(&format!("    {}({}Node),\n", node_name, node_name));
        }
        code.push_str("}\n\n");
        
        // Generate individual node structures
        for rule in &grammar.rules {
            let node_name = self.capitalize_rule_name(&rule.name);
            code.push_str(&format!("#[derive(Debug, Clone, Serialize, Deserialize)]\n"));
            code.push_str(&format!("pub struct {}Node {{\n", node_name));
            code.push_str(&format!("    pub rule_name: String,\n"));
            code.push_str(&format!("    pub production: String,\n"));
            code.push_str(&format!("    pub children: Vec<AstNode>,\n"));
            code.push_str(&format!("    pub text: Option<String>,\n"));
            code.push_str(&format!("    pub span: Option<(usize, usize)>,\n"));
            
            // Add domain-specific fields based on grammar type
            if grammar.name.contains("LLM") || grammar.name.contains("Api") {
                code.push_str(&format!("    pub parameters: Option<HashMap<String, String>>,\n"));
                code.push_str(&format!("    pub metadata: Option<HashMap<String, serde_json::Value>>,\n"));
            }
            
            code.push_str("}\n\n");
        }
        
        // Generate parser implementation
        code.push_str(&format!("// Parser implementation for {}\n", grammar.name));
        code.push_str("#[derive(Debug)]\n");
        code.push_str(&format!("pub struct {}Parser {{\n", self.capitalize_rule_name(&grammar.name)));
        code.push_str("    grammar_name: String,\n");
        code.push_str("}\n\n");
        
        code.push_str(&format!("impl {}Parser {{\n", self.capitalize_rule_name(&grammar.name)));
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str(&format!("        Self {{ grammar_name: \"{}\".to_string() }}\n", grammar.name));
        code.push_str("    }\n\n");
        
        code.push_str("    pub fn parse(&self, input: &str) -> Result<AstNode, String> {\n");
        code.push_str("        // Basic parsing implementation - would use ANTLR runtime in production\n");
        
        // Use the first rule or start rule as the root
        let root_rule = if !grammar.rules.is_empty() {
            &grammar.rules[0]
        } else {
            // Fallback if no rules
            return Ok(format!("        Ok(AstNode::Default)"));
        };
        
        let root_node_name = self.capitalize_rule_name(&root_rule.name);
        code.push_str(&format!("        let root = AstNode::{}({}Node {{\n", root_node_name, root_node_name));
        code.push_str("            rule_name: \"program\".to_string(),\n");
        code.push_str("            production: \"start\".to_string(),\n");
        code.push_str("            children: vec![],\n");
        code.push_str("            text: Some(input.to_string()),\n");
        code.push_str("            span: Some((0, input.len())),\n");
        if grammar.name.contains("LLM") || grammar.name.contains("Api") {
            code.push_str("            parameters: Some(HashMap::new()),\n");
            code.push_str("            metadata: Some(HashMap::new()),\n");
        }
        code.push_str("        });\n");
        code.push_str("        Ok(root)\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        // Add rule documentation
        code.push_str("// Grammar Rules:\n");
        for rule in &grammar.rules {
            code.push_str(&format!("// {}: {}\n", rule.name, rule.production));
            if let Some(action) = &rule.action {
                code.push_str(&format!("//   Action: {}\n", action));
            }
        }
        
        Ok(code)
    }
    
    fn capitalize_rule_name(&self, name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().replace('_', ""),
        }
    }

    fn generate_typescript_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("// Generated TypeScript parser for grammar: {}\n\n", grammar.name));
        
        // Type definitions
        code.push_str("// AST Node Types\n");
        code.push_str("export interface AstNode {\n");
        code.push_str("  type: string;\n");
        code.push_str("  text?: string;\n");
        code.push_str("  children: AstNode[];\n");
        code.push_str("  span?: [number, number];\n");
        
        if grammar.name.contains("LLM") || grammar.name.contains("Api") {
            code.push_str("  parameters?: Record<string, any>;\n");
            code.push_str("  metadata?: Record<string, any>;\n");
        }
        
        code.push_str("}\n\n");
        
        // Generate specific node interfaces
        for rule in &grammar.rules {
            let interface_name = format!("{}Node", self.capitalize_rule_name(&rule.name));
            code.push_str(&format!("export interface {} extends AstNode {{\n", interface_name));
            code.push_str(&format!("  type: '{}';\n", rule.name));
            code.push_str(&format!("  production: '{}';\n", rule.production));
            code.push_str("}\n\n");
        }
        
        // Parser class
        code.push_str(&format!("export class {}Parser {{\n", self.capitalize_rule_name(&grammar.name)));
        code.push_str(&format!("  private grammarName = '{}';\n\n", grammar.name));
        
        code.push_str("  parse(input: string): AstNode {\n");
        code.push_str("    // Basic parsing implementation - would use ANTLR runtime in production\n");
        code.push_str("    return {\n");
        code.push_str("      type: 'program',\n");
        code.push_str("      text: input,\n");
        code.push_str("      children: [],\n");
        code.push_str("      span: [0, input.length],\n");
        if grammar.name.contains("LLM") || grammar.name.contains("Api") {
            code.push_str("      parameters: {},\n");
            code.push_str("      metadata: {},\n");
        }
        code.push_str("    };\n");
        code.push_str("  }\n\n");
        
        code.push_str("  validate(node: AstNode): boolean {\n");
        code.push_str("    // Validation logic would go here\n");
        code.push_str("    return node.type !== undefined;\n");
        code.push_str("  }\n\n");
        
        code.push_str("  getGrammarInfo(): { name: string; rules: string[] } {\n");
        code.push_str("    return {\n");
        code.push_str(&format!("      name: '{}',\n", grammar.name));
        code.push_str("      rules: [\n");
        for rule in &grammar.rules {
            code.push_str(&format!("        '{}',\n", rule.name));
        }
        code.push_str("      ],\n");
        code.push_str("    };\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");
        
        // Export default
        code.push_str(&format!("export default {}Parser;\n", self.capitalize_rule_name(&grammar.name)));
        
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
        code.push_str("%{\n");
        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <string.h>\n\n");
        
        // Add domain-specific headers
        if grammar.name.contains("resource") || grammar.name.contains("Resource") {
            code.push_str("#include <math.h>\n");
            code.push_str("#include <sys/resource.h>\n");
        }
        
        if grammar.name.contains("query") || grammar.name.contains("Query") {
            code.push_str("#include <sqlite3.h>\n");
        }
        
        code.push_str("\n// Forward declarations\n");
        code.push_str("int yylex(void);\n");
        code.push_str("void yyerror(const char *s);\n\n");
        
        // Add domain-specific structures
        if grammar.name.contains("resource") || grammar.name.contains("Resource") {
            code.push_str("// Resource management structures\n");
            code.push_str("typedef struct {\n");
            code.push_str("    char *name;\n");
            code.push_str("    double value;\n");
            code.push_str("    int allocated;\n");
            code.push_str("} Resource;\n\n");
        }
        
        code.push_str("%}\n\n");
        
        // Token declarations
        code.push_str("// Token declarations\n");
        code.push_str("%union {\n");
        code.push_str("    double num;\n");
        code.push_str("    char *str;\n");
        code.push_str("    int ival;\n");
        code.push_str("}\n\n");
        
        code.push_str("%token <num> NUMBER\n");
        code.push_str("%token <str> IDENTIFIER STRING\n");
        code.push_str("%token <ival> INTEGER\n\n");
        
        // Add domain-specific tokens
        if grammar.name.contains("resource") || grammar.name.contains("Resource") {
            code.push_str("// Resource management tokens\n");
            code.push_str("%token ALLOCATE DEALLOCATE RESOURCE AGENT\n");
            code.push_str("%token CPU MEMORY GPU LOAD OPTIMIZE\n\n");
        }
        
        if grammar.name.contains("query") || grammar.name.contains("Query") {
            code.push_str("// Query language tokens\n");
            code.push_str("%token SELECT FROM WHERE LIMIT ORDER BY\n");
            code.push_str("%token MODEL INFERENCE EMBEDDING COMPLETION\n\n");
        }
        
        // Precedence declarations
        code.push_str("// Operator precedence\n");
        code.push_str("%left OR\n");
        code.push_str("%left AND\n");
        code.push_str("%right NOT\n");
        code.push_str("%left EQ NE LT LE GT GE\n");
        code.push_str("%left '+' '-'\n");
        code.push_str("%left '*' '/' '%'\n");
        code.push_str("%right '^'\n");
        code.push_str("%right UMINUS\n\n");
        
        code.push_str("%%\n\n");
        
        // Grammar rules
        code.push_str("// Grammar rules\n");
        for rule in &grammar.rules {
            code.push_str(&format!("{} : {} ", rule.name, rule.production));
            
            if let Some(action) = &rule.action {
                code.push_str(&format!("{{ {} }}", action));
            } else {
                // Generate default action based on rule type
                if rule.name.contains("expression") || rule.name.contains("expr") {
                    code.push_str("{ printf(\"Expression evaluated\\n\"); }");
                } else if rule.name.contains("statement") || rule.name.contains("stmt") {
                    code.push_str("{ printf(\"Statement executed\\n\"); }");
                } else {
                    code.push_str("{ printf(\"Rule matched: ");
                    code.push_str(&rule.name);
                    code.push_str("\\n\"); }");
                }
            }
            code.push_str(";\n");
        }
        
        code.push_str("\n%%\n\n");
        
        // Error handling
        code.push_str("void yyerror(const char *s) {\n");
        code.push_str("    fprintf(stderr, \"Parser Error: %s\\n\", s);\n");
        code.push_str("}\n\n");
        
        // Main function
        code.push_str("int main(void) {\n");
        code.push_str(&format!("    printf(\"Starting {} Parser\\n\");\n", grammar.name));
        code.push_str("    return yyparse();\n");
        code.push_str("}\n");
        
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
        code.push_str("\\documentclass[11pt]{{article}}\n");
        code.push_str("\\usepackage{{oz}}\n");
        code.push_str("\\usepackage{{zed-csp}}\n");
        code.push_str("\\usepackage{{amsmath}}\n");
        code.push_str("\\usepackage{{amssymb}}\n");
        code.push_str("\\usepackage{{theorem}}\n\n");
        code.push_str(&format!("\\title{{{} Formal Specification}}\n", grammar.name));
        code.push_str("\\author{{Generated by Grammar Service}}\n");
        code.push_str("\\date{{\\today}}\n\n");
        code.push_str("\\begin{{document}}\n");
        code.push_str("\\maketitle\n\n");
        
        code.push_str("\\section{{Introduction}}\n");
        code.push_str(&format!("This document presents the formal Z++ specification for {}.\n\n", grammar.name));
        
        // Add domain-specific introduction
        if grammar.name.contains("LLM") || grammar.name.contains("Inference") {
            code.push_str("This specification models the behavior and properties of Large Language Model ");
            code.push_str("inference systems, including request processing, resource management, and ");
            code.push_str("system invariants.\n\n");
        }
        
        code.push_str("\\section{{Formal Specifications}}\n\n");
        
        for rule in &grammar.rules {
            // Determine if this is a schema definition
            if rule.production.contains(':') && rule.production.contains("ℕ") {
                code.push_str(&format!("\\begin{{schema}}{{{}}}\n", rule.name));
                
                // Format the schema content
                let formatted_production = rule.production
                    .replace("ℕ", "\\nat")
                    .replace("ℝ", "\\real")
                    .replace("𝔽", "\\finset")
                    .replace("ℤ", "\\integer")
                    .replace("𝔹", "\\bool")
                    .replace("ℙ", "\\power")
                    .replace("⤔", "\\pfun")
                    .replace("→", "\\fun")
                    .replace("↦", "\\mapsto")
                    .replace("∈", "\\in")
                    .replace("∉", "\\notin")
                    .replace("∀", "\\forall")
                    .replace("∃", "\\exists")
                    .replace("⇒", "\\implies")
                    .replace("∧", "\\land")
                    .replace("∨", "\\lor")
                    .replace("¬", "\\lnot")
                    .replace("≤", "\\leq")
                    .replace("≥", "\\geq")
                    .replace("≠", "\\neq")
                    .replace("∅", "\\emptyset")
                    .replace("⟨", "\\langle")
                    .replace("⟩", "\\rangle")
                    .replace("⋃", "\\bigcup")
                    .replace("⋂", "\\bigcap")
                    .replace("Σ", "\\sum")
                    .replace("Δ", "\\Delta")
                    .replace("Ξ", "\\Xi");
                
                code.push_str(&format!("{}\n", formatted_production));
                code.push_str("\\end{{schema}}\n\n");
            } else {
                // Handle non-schema definitions
                code.push_str(&format!("\\subsection{{{}}}\n", rule.name));
                code.push_str(&format!("\\[{} \\defs {}\\]\n\n", rule.name, rule.production));
            }
            
            if let Some(action) = &rule.action {
                code.push_str(&format!("\\textbf{{Semantic Action:}} {}\n\n", action));
            }
        }
        
        // Add theorems section if this is a system specification
        if grammar.name.contains("System") || grammar.name.contains("Workflow") {
            code.push_str("\\section{{Theorems and Properties}}\n\n");
            code.push_str("\\begin{{theorem}}[Safety]\n");
            code.push_str("The system maintains its safety invariants at all times.\n");
            code.push_str("\\end{{theorem}}\n\n");
            
            code.push_str("\\begin{{theorem}}[Liveness]\n");
            code.push_str("The system makes progress and does not deadlock.\n");
            code.push_str("\\end{{theorem}}\n\n");
            
            code.push_str("\\begin{{theorem}}[Correctness]\n");
            code.push_str("All operations preserve system consistency.\n");
            code.push_str("\\end{{theorem}}\n\n");
        }
        
        code.push_str("\\end{{document}}\n");
        Ok(code)
    }

    fn generate_markdown_code(&self, grammar: &GrammarDefinition) -> Result<String> {
        let mut code = String::new();
        code.push_str(&format!("# Z++ Specification: {}\n\n", grammar.name));
        
        // Add table of contents
        code.push_str("## Table of Contents\n\n");
        code.push_str("1. [Overview](#overview)\n");
        code.push_str("2. [Schemas](#schemas)\n");
        code.push_str("3. [Operations](#operations)\n");
        code.push_str("4. [Invariants](#invariants)\n");
        code.push_str("5. [Theorems](#theorems)\n\n");
        
        code.push_str("## Overview\n\n");
        if grammar.name.contains("LLM") || grammar.name.contains("Inference") {
            code.push_str("This Z++ specification models the formal behavior of a Large Language Model ");
            code.push_str("inference system. It defines the mathematical properties, state transitions, ");
            code.push_str("and safety/liveness properties that govern the system's operation.\n\n");
        } else if grammar.name.contains("System") {
            code.push_str("This Z++ specification provides a formal mathematical model of the system, ");
            code.push_str("including its state space, operations, and correctness properties.\n\n");
        }
        
        code.push_str("## Schemas\n\n");
        
        let mut schema_rules = Vec::new();
        let mut operation_rules = Vec::new();
        let mut theorem_rules = Vec::new();
        
        // Categorize rules
        for rule in &grammar.rules {
            if rule.name.contains("schema") || rule.name.contains("Schema") {
                schema_rules.push(rule);
            } else if rule.name.contains("theorem") || rule.name.contains("Theorem") {
                theorem_rules.push(rule);
            } else {
                operation_rules.push(rule);
            }
        }
        
        // Generate schema documentation
        for rule in &schema_rules {
            code.push_str(&format!("### {}\n\n", rule.name));
            
            code.push_str("```z\n");
            code.push_str(&format!("schema {}\n", rule.name));
            
            // Format Z++ notation for better readability
            let formatted_production = rule.production
                .replace("ℕ", "ℕ")  // Natural numbers
                .replace("ℝ", "ℝ")  // Real numbers
                .replace("𝔽", "𝔽")  // Finite sets
                .replace("ℤ", "ℤ")  // Integers
                .replace("𝔹", "𝔹")  // Booleans
                .replace("ℙ", "ℙ")  // Power sets
                .replace(";", "\n  ");  // Format declarations on separate lines
                
            code.push_str(&format!("  {}\n", formatted_production));
            code.push_str("```\n\n");
            
            if let Some(action) = &rule.action {
                code.push_str(&format!("**Semantic Action:** `{}`\n\n", action));
            }
        }
        
        if !operation_rules.is_empty() {
            code.push_str("## Operations\n\n");
            
            for rule in &operation_rules {
                code.push_str(&format!("### {}\n\n", rule.name));
                
                code.push_str("```z\n");
                code.push_str(&rule.production);
                code.push_str("\n```\n\n");
                
                // Add operation description based on name patterns
                if rule.name.contains("Init") {
                    code.push_str("*Initialization operation that sets up the initial system state.*\n\n");
                } else if rule.name.contains("Register") {
                    code.push_str("*Registration operation for adding new entities to the system.*\n\n");
                } else if rule.name.contains("Process") {
                    code.push_str("*Processing operation that handles system requests or commands.*\n\n");
                } else if rule.name.contains("Update") {
                    code.push_str("*Update operation that modifies existing system state.*\n\n");
                }
                
                if let Some(action) = &rule.action {
                    code.push_str(&format!("**Implementation Notes:** {}\n\n", action));
                }
            }
        }
        
        code.push_str("## Invariants\n\n");
        code.push_str("The following invariants must hold at all times:\n\n");
        
        // Extract invariants from rule names and productions
        for rule in &grammar.rules {
            if rule.name.contains("Invariant") || rule.production.contains("≤") || rule.production.contains("≥") {
                code.push_str(&format!("- **{}**: {}\n", rule.name, rule.production));
            }
        }
        
        if !theorem_rules.is_empty() {
            code.push_str("\n## Theorems\n\n");
            
            for rule in &theorem_rules {
                code.push_str(&format!("### {}\n\n", rule.name));
                code.push_str(&format!("```z\n{}\n```\n\n", rule.production));
                
                if rule.name.contains("Safety") {
                    code.push_str("*This theorem ensures that the system maintains safety properties.*\n\n");
                } else if rule.name.contains("Liveness") {
                    code.push_str("*This theorem guarantees system progress and absence of deadlocks.*\n\n");
                } else if rule.name.contains("Correctness") {
                    code.push_str("*This theorem proves the correctness of system operations.*\n\n");
                }
            }
        }
        
        // Add notation guide
        code.push_str("## Z++ Notation Guide\n\n");
        code.push_str("| Symbol | Meaning |\n");
        code.push_str("|--------|----------|\n");
        code.push_str("| ℕ | Natural numbers |\n");
        code.push_str("| ℝ | Real numbers |\n");
        code.push_str("| ℤ | Integers |\n");
        code.push_str("| 𝔹 | Boolean values |\n");
        code.push_str("| 𝔽 | Finite sets |\n");
        code.push_str("| ℙ | Power set |\n");
        code.push_str("| → | Total function |\n");
        code.push_str("| ⤔ | Partial function |\n");
        code.push_str("| ∈ | Set membership |\n");
        code.push_str("| ∀ | Universal quantifier |\n");
        code.push_str("| ∃ | Existential quantifier |\n");
        code.push_str("| ⇒ | Implication |\n");
        code.push_str("| ∧ | Logical AND |\n");
        code.push_str("| ∨ | Logical OR |\n");
        code.push_str("| Δ | State change |\n");
        code.push_str("| Ξ | No state change |\n\n");
        
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