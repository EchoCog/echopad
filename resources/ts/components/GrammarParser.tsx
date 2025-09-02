import React, { useState } from 'react';

export interface GrammarParserProps {
  // Optional props for customization
  className?: string;
}

interface ParseRequest {
  grammar_name: string;
  input: string;
}

interface LoadGrammarRequest {
  name: string;
  grammar_type: string;
  content: string;
}

interface GenerateCodeRequest {
  grammar_name: string;
  target_language: string;
}

export const GrammarParser: React.FC<GrammarParserProps> = ({ className = "" }) => {
  const [grammarName, setGrammarName] = useState('ArithmeticGrammar');
  const [input, setInput] = useState('2 + 3 * 4');
  const [parseResult, setParseResult] = useState<string>('');
  const [availableGrammars, setAvailableGrammars] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // Load a new grammar
  const [newGrammarName, setNewGrammarName] = useState('');
  const [grammarType, setGrammarType] = useState<string>('antlr');
  const [grammarContent, setGrammarContent] = useState('');

  // Code generation
  const [targetLanguage, setTargetLanguage] = useState<string>('rust');
  const [generatedCode, setGeneratedCode] = useState<string>('');

  const parseInput = async () => {
    setIsLoading(true);
    try {
      const request: ParseRequest = {
        grammar_name: grammarName,
        input: input
      };

      const response = await fetch('/api/grammar/parse', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      });

      const result = await response.json();
      setParseResult(JSON.stringify(result, null, 2));
    } catch (error) {
      setParseResult(`Error: ${error}`);
    }
    setIsLoading(false);
  };

  const loadGrammar = async () => {
    setIsLoading(true);
    try {
      const request: LoadGrammarRequest = {
        name: newGrammarName,
        grammar_type: grammarType,
        content: grammarContent
      };

      const response = await fetch('/api/grammar/load', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      });

      const result = await response.json();
      setParseResult(JSON.stringify(result, null, 2));
      
      // Refresh grammar list
      await listGrammars();
    } catch (error) {
      setParseResult(`Error: ${error}`);
    }
    setIsLoading(false);
  };

  const generateCode = async () => {
    setIsLoading(true);
    try {
      const request: GenerateCodeRequest = {
        grammar_name: grammarName,
        target_language: targetLanguage
      };

      const response = await fetch('/api/grammar/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      });

      const result = await response.json();
      if (result.success && result.code) {
        setGeneratedCode(result.code);
      } else {
        setGeneratedCode(`Error: ${result.error || 'Unknown error'}`);
      }
    } catch (error) {
      setGeneratedCode(`Error: ${error}`);
    }
    setIsLoading(false);
  };

  const listGrammars = async () => {
    try {
      const response = await fetch('/api/grammar/list');
      const grammars = await response.json();
      setAvailableGrammars(grammars);
    } catch (error) {
      console.error('Error loading grammars:', error);
    }
  };

  // Load grammars on component mount
  React.useEffect(() => {
    listGrammars();
  }, []);

  return (
    <div className={`grammar-parser-container ${className}`}>
      <div className="section">
        <h2>Grammar Parser</h2>
        
        <div className="form-group">
          <label htmlFor="grammar-select">Select Grammar:</label>
          <select 
            id="grammar-select"
            value={grammarName} 
            onChange={(e) => setGrammarName(e.target.value)}
          >
            {availableGrammars.map(name => (
              <option key={name} value={name}>{name}</option>
            ))}
          </select>
          <button onClick={listGrammars} disabled={isLoading}>
            Refresh List
          </button>
        </div>

        <div className="form-group">
          <label htmlFor="input-text">Input to Parse:</label>
          <textarea 
            id="input-text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            rows={3}
            placeholder="Enter text to parse..."
          />
        </div>

        <button onClick={parseInput} disabled={isLoading}>
          {isLoading ? 'Parsing...' : 'Parse Input'}
        </button>

        <div className="result-section">
          <h3>Parse Result:</h3>
          <pre>{parseResult}</pre>
        </div>
      </div>

      <div className="section">
        <h2>Load Grammar</h2>
        
        <div className="form-group">
          <label htmlFor="new-grammar-name">Grammar Name:</label>
          <input 
            id="new-grammar-name"
            type="text"
            value={newGrammarName}
            onChange={(e) => setNewGrammarName(e.target.value)}
            placeholder="MyGrammar"
          />
        </div>

        <div className="form-group">
          <label htmlFor="grammar-type-select">Grammar Type:</label>
          <select 
            id="grammar-type-select"
            value={grammarType}
            onChange={(e) => setGrammarType(e.target.value)}
          >
            <option value="antlr">ANTLR</option>
            <option value="yacc">YACC</option>
            <option value="z++">Z++</option>
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="grammar-content">Grammar Content:</label>
          <textarea 
            id="grammar-content"
            value={grammarContent}
            onChange={(e) => setGrammarContent(e.target.value)}
            rows={6}
            placeholder="Enter grammar definition..."
          />
        </div>

        <button onClick={loadGrammar} disabled={isLoading || !newGrammarName || !grammarContent}>
          {isLoading ? 'Loading...' : 'Load Grammar'}
        </button>
      </div>

      <div className="section">
        <h2>Code Generation</h2>
        
        <div className="form-group">
          <label htmlFor="target-language">Target Language:</label>
          <select 
            id="target-language"
            value={targetLanguage}
            onChange={(e) => setTargetLanguage(e.target.value)}
          >
            <option value="rust">Rust</option>
            <option value="typescript">TypeScript</option>
            <option value="c">C</option>
            <option value="latex">LaTeX</option>
            <option value="markdown">Markdown</option>
          </select>
        </div>

        <button onClick={generateCode} disabled={isLoading}>
          {isLoading ? 'Generating...' : 'Generate Code'}
        </button>

        <div className="result-section">
          <h3>Generated Code:</h3>
          <pre>{generatedCode}</pre>
        </div>
      </div>

      <style jsx>{`
        .grammar-parser-container {
          padding: 20px;
          max-width: 800px;
        }
        
        .section {
          margin-bottom: 30px;
          border: 1px solid #ddd;
          padding: 20px;
          border-radius: 8px;
        }
        
        .form-group {
          margin-bottom: 15px;
        }
        
        label {
          display: block;
          margin-bottom: 5px;
          font-weight: bold;
        }
        
        input, select, textarea {
          width: 100%;
          padding: 8px;
          border: 1px solid #ccc;
          border-radius: 4px;
          font-family: monospace;
        }
        
        button {
          background-color: #007bff;
          color: white;
          padding: 10px 20px;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          margin-right: 10px;
        }
        
        button:disabled {
          background-color: #6c757d;
          cursor: not-allowed;
        }
        
        button:hover:not(:disabled) {
          background-color: #0056b3;
        }
        
        .result-section {
          margin-top: 20px;
        }
        
        pre {
          background-color: #f8f9fa;
          border: 1px solid #e9ecef;
          border-radius: 4px;
          padding: 10px;
          white-space: pre-wrap;
          max-height: 300px;
          overflow-y: auto;
          font-family: 'Courier New', monospace;
        }
      `}</style>
    </div>
  );
};