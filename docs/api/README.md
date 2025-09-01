# API Documentation

This section provides comprehensive documentation for Paddler's APIs, including client-facing inference APIs and internal management APIs.

## API Overview

Paddler exposes multiple API interfaces for different use cases:

```mermaid
graph TB
    subgraph "External APIs"
        INFERENCE[Inference API<br/>OpenAI Compatible]
        MANAGEMENT[Management API<br/>REST + WebSocket]
    end
    
    subgraph "Internal APIs"
        AGENT_RPC[Agent RPC<br/>JSON-RPC over WebSocket]
        HEALTH[Health Check API<br/>HTTP]
        METRICS[Metrics API<br/>Prometheus Format]
    end
    
    subgraph "Clients"
        CLIENT_APPS[Client Applications]
        ADMIN_UI[Admin Web UI]
        MONITORING[Monitoring Tools]
        AGENTS[Agent Processes]
    end
    
    CLIENT_APPS --> INFERENCE
    ADMIN_UI --> MANAGEMENT
    MONITORING --> HEALTH
    MONITORING --> METRICS
    AGENTS --> AGENT_RPC
    
    style INFERENCE fill:#e3f2fd
    style MANAGEMENT fill:#fff3e0
    style AGENT_RPC fill:#f3e5f5
```

## API Structure

### Inference API (Client-Facing)

**Base URL**: `http://balancer:8080`

OpenAI-compatible endpoints for LLM inference:

```mermaid
graph LR
    subgraph "Inference Endpoints"
        COMPLETIONS[/v1/completions<br/>Text completion]
        CHAT[/v1/chat/completions<br/>Chat completion]
        EMBEDDINGS[/v1/embeddings<br/>Text embeddings]
        MODELS[/v1/models<br/>Available models]
    end
    
    subgraph "Features"
        STREAMING[Server-Sent Events]
        BATCHING[Batch Processing]
        ASYNC[Async Responses]
    end
    
    COMPLETIONS --> STREAMING
    CHAT --> STREAMING
    EMBEDDINGS --> BATCHING
    MODELS --> ASYNC
```

### Management API (Administrative)

**Base URL**: `http://balancer:8081`

Administrative control and monitoring:

```mermaid
graph LR
    subgraph "Management Endpoints"
        AGENTS[/agents<br/>Agent management]
        MODELS_MGMT[/models<br/>Model configuration]
        HEALTH_MGMT[/health<br/>System health]
        CONFIG[/config<br/>System configuration]
    end
    
    subgraph "Real-time APIs"
        WS_CONTROL[WebSocket Control<br/>Live updates]
        WS_LOGS[WebSocket Logs<br/>Log streaming]
        WS_METRICS[WebSocket Metrics<br/>Live metrics]
    end
    
    AGENTS --> WS_CONTROL
    MODELS_MGMT --> WS_CONTROL
    HEALTH_MGMT --> WS_METRICS
    CONFIG --> WS_LOGS
```

## Authentication and Security

### API Authentication

```mermaid
sequenceDiagram
    participant Client
    participant Balancer
    participant AuthService
    
    Client->>Balancer: Request + API Key
    Balancer->>AuthService: Validate Key
    AuthService->>Balancer: Validation Result
    
    alt Valid Key
        Balancer->>Client: Process Request
    else Invalid Key
        Balancer->>Client: 401 Unauthorized
    end
```

### Security Headers

All API responses include security headers:
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security: max-age=31536000`

## Protocol Specifications

### HTTP/REST API

Standard HTTP methods with JSON payloads:

```mermaid
graph LR
    subgraph "HTTP Methods"
        GET[GET<br/>Retrieve resources]
        POST[POST<br/>Create/execute]
        PUT[PUT<br/>Update resources]
        DELETE[DELETE<br/>Remove resources]
        PATCH[PATCH<br/>Partial updates]
    end
    
    subgraph "Response Formats"
        JSON[JSON<br/>Standard responses]
        SSE[Server-Sent Events<br/>Streaming]
        BINARY[Binary<br/>Model files]
    end
    
    GET --> JSON
    POST --> JSON
    POST --> SSE
    PUT --> JSON
    DELETE --> JSON
    PATCH --> JSON
    
    JSON --> BINARY
```

### WebSocket API

Real-time bidirectional communication:

```mermaid
sequenceDiagram
    participant Client
    participant Balancer
    
    Client->>Balancer: WebSocket Upgrade
    Balancer->>Client: Connection Established
    
    loop Real-time Communication
        Client->>Balancer: JSON Message
        Balancer->>Client: JSON Response
        Balancer->>Client: Event Notification
    end
    
    Note over Client,Balancer: Heartbeat every 30s
    Client->>Balancer: Ping
    Balancer->>Client: Pong
```

### JSON-RPC Protocol

Internal agent communication uses JSON-RPC 2.0:

```json
{
  "jsonrpc": "2.0",
  "method": "generate_tokens",
  "params": {
    "prompt": "Hello, world!",
    "max_tokens": 100,
    "temperature": 0.7
  },
  "id": "request-123"
}
```

## Error Handling

### HTTP Status Codes

```mermaid
graph LR
    subgraph "Success (2xx)"
        S200[200 OK<br/>Successful request]
        S201[201 Created<br/>Resource created]
        S202[202 Accepted<br/>Async processing]
    end
    
    subgraph "Client Errors (4xx)"
        E400[400 Bad Request<br/>Invalid input]
        E401[401 Unauthorized<br/>Authentication failed]
        E404[404 Not Found<br/>Resource not found]
        E429[429 Too Many Requests<br/>Rate limited]
    end
    
    subgraph "Server Errors (5xx)"
        E500[500 Internal Error<br/>Server problem]
        E503[503 Service Unavailable<br/>No agents available]
        E504[504 Gateway Timeout<br/>Request timeout]
    end
```

### Error Response Format

```json
{
  "error": {
    "code": "invalid_request",
    "message": "The request is missing required parameter 'prompt'",
    "details": {
      "parameter": "prompt",
      "received_value": null,
      "expected_type": "string"
    },
    "request_id": "req_123456789"
  }
}
```

## Rate Limiting and Quotas

### Rate Limiting Strategy

```mermaid
graph TB
    REQUEST[Incoming Request] --> IDENTIFY[Identify Client]
    IDENTIFY --> CHECK_LIMIT{Within Limits?}
    
    CHECK_LIMIT -->|Yes| PROCESS[Process Request]
    CHECK_LIMIT -->|No| RATE_LIMIT[Return 429]
    
    subgraph "Limit Types"
        RPM[Requests per Minute]
        TPM[Tokens per Minute]
        CONCURRENT[Concurrent Requests]
    end
    
    CHECK_LIMIT --> RPM
    CHECK_LIMIT --> TPM
    CHECK_LIMIT --> CONCURRENT
    
    PROCESS --> SUCCESS[Success Response]
    RATE_LIMIT --> RETRY_AFTER[Include Retry-After Header]
```

### Quota Management

Different quota tiers based on client classification:

```mermaid
graph LR
    subgraph "Client Tiers"
        FREE[Free Tier<br/>1000 requests/day]
        BASIC[Basic Tier<br/>10K requests/day]
        PRO[Pro Tier<br/>100K requests/day]
        ENTERPRISE[Enterprise<br/>Unlimited]
    end
    
    subgraph "Features"
        RATE_LIMITS[Rate Limits]
        PRIORITY[Request Priority]
        SLA[SLA Guarantees]
        SUPPORT[Support Level]
    end
    
    FREE --> RATE_LIMITS
    BASIC --> RATE_LIMITS
    PRO --> PRIORITY
    ENTERPRISE --> SLA
    ENTERPRISE --> SUPPORT
```

## API Documentation Structure

- **[Inference API](./inference-api.md)** - OpenAI-compatible inference endpoints
- **[Management API](./management-api.md)** - Administrative and configuration APIs
- **[WebSocket API](./websocket-api.md)** - Real-time communication protocols
- **[Internal APIs](./internal-apis.md)** - Agent communication and system APIs
- **[API Examples](./examples/)** - Code examples and tutorials
- **[API Reference](./reference/)** - Complete API specification

## OpenAPI Specification

Paddler provides OpenAPI 3.0 specifications for all APIs:

```yaml
openapi: 3.0.0
info:
  title: Paddler API
  version: 2.1.1
  description: LLMOps platform for hosting and scaling open-source LLMs
  contact:
    name: Intentee
    url: https://paddler.intentee.com
servers:
  - url: http://localhost:8080
    description: Local development server
  - url: https://api.paddler.example.com
    description: Production server
```

## SDK and Client Libraries

### Official SDKs

```mermaid
graph TB
    subgraph "Official SDKs"
        PYTHON[Python SDK<br/>paddler-python]
        NODE[Node.js SDK<br/>paddler-js]
        GO[Go SDK<br/>paddler-go]
        RUST[Rust SDK<br/>paddler-rs]
    end
    
    subgraph "Third-party Compatible"
        OPENAI_PYTHON[OpenAI Python<br/>Compatible]
        OPENAI_NODE[OpenAI Node.js<br/>Compatible]
        LANGCHAIN[LangChain<br/>Integration]
    end
    
    PYTHON --> OPENAI_PYTHON
    NODE --> OPENAI_NODE
    PYTHON --> LANGCHAIN
```

### Example Usage

```python
import paddler

# Initialize client
client = paddler.Client(
    api_key="your-api-key",
    base_url="http://paddler.example.com"
)

# Generate completion
response = client.completions.create(
    model="llama-7b",
    prompt="Hello, world!",
    max_tokens=100,
    stream=True
)

# Stream tokens
for token in response:
    print(token.choices[0].text, end="")
```

## Performance and Optimization

### Request Optimization

```mermaid
graph LR
    subgraph "Client Optimizations"
        BATCHING[Request Batching]
        CACHING[Response Caching]
        COMPRESSION[Request Compression]
        POOLING[Connection Pooling]
    end
    
    subgraph "Server Optimizations"
        ASYNC[Async Processing]
        STREAMING[Response Streaming]
        PREFETCH[Model Prefetching]
        LOAD_BALANCE[Load Balancing]
    end
    
    BATCHING --> ASYNC
    CACHING --> PREFETCH
    COMPRESSION --> STREAMING
    POOLING --> LOAD_BALANCE
```

### API Versioning

```mermaid
graph TB
    subgraph "Versioning Strategy"
        V1[API v1<br/>Stable, deprecated]
        V2[API v2<br/>Current stable]
        V3[API v3<br/>Beta features]
    end
    
    subgraph "Migration Path"
        DEPRECATION[Deprecation Notice]
        COMPATIBILITY[Backward Compatibility]
        SUNSET[Sunset Timeline]
    end
    
    V1 --> DEPRECATION
    V2 --> COMPATIBILITY
    V3 --> COMPATIBILITY
    
    DEPRECATION --> SUNSET
```

Version is specified in URL path: `/v1/completions`, `/v2/completions`

## Monitoring and Analytics

### API Metrics

```mermaid
graph LR
    subgraph "Request Metrics"
        THROUGHPUT[Requests/Second]
        LATENCY[Response Latency]
        ERROR_RATE[Error Rate]
        SUCCESS_RATE[Success Rate]
    end
    
    subgraph "Resource Metrics"
        TOKEN_RATE[Tokens/Second]
        QUEUE_DEPTH[Queue Depth]
        AGENT_UTIL[Agent Utilization]
        MODEL_LOAD[Model Load Time]
    end
    
    THROUGHPUT --> DASHBOARD[API Dashboard]
    LATENCY --> DASHBOARD
    ERROR_RATE --> DASHBOARD
    TOKEN_RATE --> DASHBOARD
```

### Request Tracing

Each API request gets a unique trace ID for end-to-end tracking:

```mermaid
sequenceDiagram
    participant Client
    participant Balancer
    participant Agent
    participant LlamaCpp
    
    Note over Client,LlamaCpp: Trace ID: trace-abc123
    
    Client->>Balancer: Request (trace-abc123)
    Balancer->>Agent: Forward (trace-abc123)
    Agent->>LlamaCpp: Process (trace-abc123)
    LlamaCpp->>Agent: Response (trace-abc123)
    Agent->>Balancer: Response (trace-abc123)
    Balancer->>Client: Response (trace-abc123)
```