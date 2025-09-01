# Core Components

This document provides detailed information about Paddler's core components and their responsibilities.

## Balancer Components

### Architecture Overview

```mermaid
graph TB
    subgraph "Balancer Process"
        subgraph "HTTP Services"
            INFERENCE[Inference Service<br/>:8080]
            MGMT[Management Service<br/>:8081]
            WEB[Web Admin Panel<br/>:8080/ui]
        end
        
        subgraph "Core Services"
            BUFFER[Request Buffer Manager]
            AGENT_POOL[Agent Controller Pool]
            STATE_DB[State Database]
            RECONCILE[Reconciliation Service]
        end
        
        subgraph "Communication"
            WS_CTRL[WebSocket Controller]
            RPC[JSON-RPC Handler]
        end
    end
    
    INFERENCE --> BUFFER
    BUFFER --> AGENT_POOL
    AGENT_POOL --> WS_CTRL
    WS_CTRL --> RPC
    
    MGMT --> STATE_DB
    STATE_DB --> RECONCILE
    RECONCILE --> AGENT_POOL
    
    WEB --> MGMT
    WEB --> INFERENCE
```

### Inference Service

**Purpose**: Handles client requests for token generation and embeddings.

**Key Responsibilities**:
- Accept HTTP requests for inference
- Validate request parameters
- Queue requests in buffer manager
- Stream responses back to clients
- Handle both completion and embedding requests

**API Endpoints**:
- `POST /v1/completions` - OpenAI-compatible completions
- `POST /v1/embeddings` - OpenAI-compatible embeddings
- `POST /v1/chat/completions` - OpenAI-compatible chat completions

### Management Service

**Purpose**: Provides administrative control over the Paddler system.

**Key Responsibilities**:
- Agent registration and health monitoring
- Model metadata management
- System configuration
- State synchronization
- Performance metrics collection

**API Endpoints**:
- `GET /agents` - List registered agents
- `POST /models` - Add/update model configurations
- `GET /metrics` - System performance metrics
- `GET /health` - System health status

### Request Buffer Manager

**Purpose**: Queues and manages incoming inference requests.

```mermaid
graph LR
    subgraph "Request Buffer"
        QUEUE[Request Queue]
        ROUTER[Request Router]
        MONITOR[Queue Monitor]
    end
    
    CLIENT_REQ[Client Requests] --> QUEUE
    QUEUE --> ROUTER
    ROUTER --> AGENT_SELECT[Agent Selection]
    MONITOR --> METRICS[Queue Metrics]
```

**Key Features**:
- FIFO request queuing
- Priority-based routing
- Queue overflow protection
- Request timeout handling
- Zero-agent graceful queuing

### Agent Controller Pool

**Purpose**: Manages connections and communication with agent fleet.

**Key Responsibilities**:
- Maintain WebSocket connections to agents
- Load balancing across agents
- Agent health monitoring
- Request distribution
- Response aggregation

```mermaid
graph TB
    subgraph "Agent Pool"
        POOL[Controller Pool]
        CTRL1[Agent Controller 1]
        CTRL2[Agent Controller 2]
        CTRL3[Agent Controller N]
    end
    
    POOL --> CTRL1
    POOL --> CTRL2
    POOL --> CTRL3
    
    CTRL1 --> WS1[WebSocket 1]
    CTRL2 --> WS2[WebSocket 2]
    CTRL3 --> WS3[WebSocket N]
```

## Agent Components

### Architecture Overview

```mermaid
graph TB
    subgraph "Agent Process"
        subgraph "Core Services"
            MGMT_CLIENT[Management Client]
            ARBITER[LlamaCpp Arbiter]
            RECONCILE_AGENT[Reconciliation Service]
        end
        
        subgraph "Inference Engine"
            SLOT1[Slot 1]
            SLOT2[Slot 2]
            SLOT3[Slot N]
            LLAMACPP[llama.cpp Engine]
        end
        
        subgraph "State Management"
            MODEL_META[Model Metadata]
            STATE_HOLDER[State Holder]
        end
    end
    
    MGMT_CLIENT --> RECONCILE_AGENT
    RECONCILE_AGENT --> ARBITER
    ARBITER --> SLOT1
    ARBITER --> SLOT2
    ARBITER --> SLOT3
    
    SLOT1 --> LLAMACPP
    SLOT2 --> LLAMACPP
    SLOT3 --> LLAMACPP
    
    LLAMACPP --> MODEL_META
    STATE_HOLDER --> RECONCILE_AGENT
```

### LlamaCpp Arbiter

**Purpose**: Coordinates access to the llama.cpp inference engine.

**Key Responsibilities**:
- Manage multiple inference slots
- Distribute requests across slots
- Handle model loading/unloading
- Monitor slot performance
- KV cache management

### Inference Slots

**Purpose**: Individual execution contexts for LLM inference.

**Key Features**:
- Separate KV cache per slot
- Independent context management
- Concurrent request processing
- Context length management
- Memory optimization

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Loading: Load Model
    Loading --> Ready: Model Loaded
    Ready --> Processing: Receive Request
    Processing --> Streaming: Generate Tokens
    Streaming --> Ready: Complete
    Ready --> Unloading: Unload Model
    Unloading --> Idle: Model Unloaded
    Processing --> Ready: Error/Cancel
```

### Management Socket Client

**Purpose**: Maintains connection with balancer for control and monitoring.

**Key Responsibilities**:
- WebSocket connection management
- Health status reporting
- Configuration updates
- Model synchronization
- Performance metrics reporting

## Communication Patterns

### WebSocket Communication

```mermaid
sequenceDiagram
    participant Balancer
    participant Agent
    participant LlamaCpp
    
    Note over Balancer,Agent: Connection Establishment
    Agent->>Balancer: WebSocket Connect
    Balancer->>Agent: Connection ACK
    
    Note over Balancer,Agent: Health Monitoring
    Balancer->>Agent: Health Check
    Agent->>Balancer: Status Report
    
    Note over Balancer,Agent: Request Processing
    Balancer->>Agent: Inference Request
    Agent->>LlamaCpp: Process Request
    LlamaCpp->>Agent: Token Stream
    Agent->>Balancer: Response Stream
```

### JSON-RPC Protocol

**Message Types**:
- `generate_tokens` - Token generation request
- `generate_embeddings` - Embedding generation request
- `health_status` - Health status update
- `model_update` - Model configuration change

### State Synchronization

```mermaid
graph LR
    subgraph "State Flow"
        DESIRED[Desired State]
        APPLICABLE[Applicable State]
        CURRENT[Current State]
    end
    
    DESIRED -->|Reconciliation| APPLICABLE
    APPLICABLE -->|Application| CURRENT
    CURRENT -->|Feedback| DESIRED
```

## Error Handling and Recovery

### Agent Failure Handling

1. **Connection Loss**: Automatic reconnection with exponential backoff
2. **Slot Failures**: Isolated failure, other slots continue operating
3. **Model Loading Errors**: Graceful degradation, error reporting
4. **Memory Issues**: Automatic cleanup and restart procedures

### Balancer Resilience

1. **Agent Pool Management**: Removes failed agents from active pool
2. **Request Queuing**: Buffers requests during agent unavailability
3. **Health Monitoring**: Continuous agent health verification
4. **Graceful Degradation**: Reduced capacity rather than complete failure