# Request Processing Flow

This document details how requests flow through the Paddler system from initial client request to final response.

## Complete Request Lifecycle

```mermaid
graph TD
    CLIENT[Client Application] -->|HTTP Request| INFERENCE_SVC[Inference Service]
    
    subgraph "Balancer Processing"
        INFERENCE_SVC --> VALIDATE[Request Validation]
        VALIDATE --> BUFFER[Request Buffer]
        BUFFER --> QUEUE[Request Queue]
        QUEUE --> ROUTER[Request Router]
        ROUTER --> AGENT_SELECT[Agent Selection]
        AGENT_SELECT --> FORWARD[Forward to Agent]
    end
    
    subgraph "Agent Processing"
        FORWARD --> AGENT[Agent Receiver]
        AGENT --> SLOT_SELECT[Slot Selection]
        SLOT_SELECT --> SLOT[Available Slot]
        SLOT --> LLAMACPP[llama.cpp Engine]
        LLAMACPP --> GENERATE[Token Generation]
        GENERATE --> STREAM_BACK[Stream Response]
    end
    
    STREAM_BACK --> AGGREGATE[Response Aggregation]
    AGGREGATE --> CLIENT
    
    style CLIENT fill:#e1f5fe
    style LLAMACPP fill:#f3e5f5
    style STREAM_BACK fill:#e8f5e8
```

## Detailed Request Flow

### 1. Request Reception

```mermaid
sequenceDiagram
    participant Client
    participant InferenceService
    participant RequestValidator
    participant RequestBuffer
    
    Client->>InferenceService: POST /v1/completions
    InferenceService->>RequestValidator: Validate Request
    alt Valid Request
        RequestValidator->>RequestBuffer: Queue Request
        RequestBuffer->>InferenceService: Request ID
        InferenceService->>Client: HTTP 200 + Stream
    else Invalid Request
        RequestValidator->>InferenceService: Validation Error
        InferenceService->>Client: HTTP 400 + Error
    end
```

**Request Validation Steps**:
1. JSON schema validation
2. Parameter range checks
3. Authentication verification
4. Rate limiting enforcement
5. Model availability check

### 2. Request Queuing and Routing

```mermaid
graph LR
    subgraph "Request Buffer"
        QUEUE[Request Queue]
        PRIORITY[Priority Logic]
        MONITOR[Queue Monitor]
    end
    
    subgraph "Agent Selection"
        AVAILABLE[Available Agents]
        LOAD_BALANCER[Load Balancer]
        HEALTH_CHECK[Health Filter]
    end
    
    QUEUE --> PRIORITY
    PRIORITY --> LOAD_BALANCER
    AVAILABLE --> HEALTH_CHECK
    HEALTH_CHECK --> LOAD_BALANCER
    LOAD_BALANCER --> SELECTED[Selected Agent]
    
    MONITOR --> METRICS[Queue Metrics]
```

**Selection Criteria**:
- Agent health status
- Current load/slot availability
- Model compatibility
- Geographic proximity (if applicable)
- Historical performance

### 3. Agent Request Processing

```mermaid
sequenceDiagram
    participant Balancer
    participant Agent
    participant SlotManager
    participant LlamaCpp
    participant ModelStorage
    
    Balancer->>Agent: JSON-RPC Request
    Agent->>SlotManager: Find Available Slot
    
    alt Slot Available
        SlotManager->>Agent: Slot Assignment
        Agent->>LlamaCpp: Process Request
        
        alt Model Loaded
            LlamaCpp->>Agent: Start Generation
        else Model Not Loaded
            LlamaCpp->>ModelStorage: Load Model
            ModelStorage->>LlamaCpp: Model Data
            LlamaCpp->>Agent: Start Generation
        end
        
        loop Token Generation
            LlamaCpp->>Agent: Generated Token
            Agent->>Balancer: Stream Token
            Balancer->>Client: Forward Token
        end
        
    else No Slots Available
        SlotManager->>Agent: Queue Request
        Agent->>Balancer: Request Queued
    end
```

### 4. Token Generation Flow

```mermaid
stateDiagram-v2
    [*] --> ReceiveRequest
    ReceiveRequest --> ValidateModel: Model Check
    ValidateModel --> LoadModel: Model Not Loaded
    ValidateModel --> PrepareContext: Model Ready
    LoadModel --> PrepareContext: Load Complete
    PrepareContext --> TokenizeInput: Setup Context
    TokenizeInput --> GenerateToken: Input Processed
    
    GenerateToken --> StreamToken: Token Generated
    StreamToken --> CheckCompletion: Token Sent
    CheckCompletion --> GenerateToken: Continue
    CheckCompletion --> CompleteRequest: Done
    CompleteRequest --> [*]
    
    note right of LoadModel
        Model loading can take
        several seconds for
        large models
    end note
    
    note right of GenerateToken
        Token generation is
        the main processing
        bottleneck
    end note
```

## Request Types and Processing

### Completion Requests

```mermaid
graph TD
    INPUT[Raw Prompt] --> TEMPLATE[Apply Chat Template]
    TEMPLATE --> TOKENIZE[Tokenize Input]
    TOKENIZE --> CONTEXT[Setup Context]
    CONTEXT --> GENERATE[Generate Loop]
    
    subgraph "Generation Loop"
        SAMPLE[Sample Next Token]
        SAMPLE --> CHECK[Check Stop Conditions]
        CHECK -->|Continue| SAMPLE
        CHECK -->|Complete| OUTPUT[Final Output]
    end
    
    GENERATE --> SAMPLE
    OUTPUT --> DETOKENIZE[Detokenize Response]
    DETOKENIZE --> RESPONSE[HTTP Response]
```

### Embedding Requests

```mermaid
graph TD
    INPUT[Text Input] --> TOKENIZE[Tokenize Text]
    TOKENIZE --> ENCODE[Encode with Model]
    ENCODE --> POOL[Apply Pooling]
    
    subgraph "Pooling Strategies"
        MEAN[Mean Pooling]
        CLS[CLS Token]
        LAST[Last Token]
    end
    
    POOL --> MEAN
    POOL --> CLS
    POOL --> LAST
    
    MEAN --> NORMALIZE[Normalize Vector]
    CLS --> NORMALIZE
    LAST --> NORMALIZE
    
    NORMALIZE --> RESPONSE[Embedding Response]
```

## Performance Characteristics

### Latency Breakdown

```mermaid
gantt
    title Request Processing Timeline
    dateFormat X
    axisFormat %s
    
    section Network
    Client to Balancer    :0, 10
    Balancer to Agent     :10, 15
    Agent to Client       :95, 100
    
    section Processing
    Request Validation    :5, 8
    Queue Processing      :8, 12
    Agent Selection       :12, 15
    Model Processing      :15, 95
    
    section Model Work
    Context Setup         :15, 20
    Token Generation      :20, 90
    Response Assembly     :90, 95
```

### Throughput Optimization

**Bottleneck Identification**:
1. **Model Loading**: Cache models in memory
2. **Token Generation**: Optimize batch processing
3. **Network**: Use efficient serialization
4. **Queue Processing**: Parallel request handling

**Scaling Strategies**:
- Horizontal agent scaling
- Slot multiplexing
- Request batching
- Model sharding

## Error Handling in Request Flow

```mermaid
graph TD
    REQUEST[Incoming Request] --> VALIDATE{Valid?}
    VALIDATE -->|No| ERROR_400[HTTP 400 Error]
    VALIDATE -->|Yes| QUEUE[Queue Request]
    
    QUEUE --> AGENT_SELECT{Agent Available?}
    AGENT_SELECT -->|No| ERROR_503[HTTP 503 Service Unavailable]
    AGENT_SELECT -->|Yes| FORWARD[Forward to Agent]
    
    FORWARD --> PROCESS{Processing OK?}
    PROCESS -->|Error| ERROR_500[HTTP 500 Internal Error]
    PROCESS -->|Timeout| ERROR_504[HTTP 504 Timeout]
    PROCESS -->|Success| RESPONSE[Successful Response]
    
    ERROR_400 --> LOG[Log Error]
    ERROR_503 --> LOG
    ERROR_500 --> LOG
    ERROR_504 --> LOG
    LOG --> METRICS[Update Metrics]
```

### Error Recovery Strategies

1. **Request Retry**: Automatic retry on transient failures
2. **Agent Failover**: Route to different agent on failure
3. **Graceful Degradation**: Reduced functionality vs complete failure
4. **Circuit Breaker**: Prevent cascade failures

## Monitoring and Observability

### Key Metrics

```mermaid
graph LR
    subgraph "Request Metrics"
        REQ_RATE[Request Rate]
        REQ_LATENCY[Request Latency]
        REQ_ERRORS[Error Rate]
    end
    
    subgraph "System Metrics"
        AGENT_HEALTH[Agent Health]
        SLOT_UTIL[Slot Utilization]
        QUEUE_DEPTH[Queue Depth]
    end
    
    subgraph "Model Metrics"
        TOKEN_RATE[Tokens/Second]
        MODEL_LOAD[Model Load Time]
        CONTEXT_SIZE[Context Usage]
    end
    
    REQ_RATE --> DASHBOARD[Monitoring Dashboard]
    REQ_LATENCY --> DASHBOARD
    REQ_ERRORS --> DASHBOARD
    AGENT_HEALTH --> DASHBOARD
    SLOT_UTIL --> DASHBOARD
    QUEUE_DEPTH --> DASHBOARD
    TOKEN_RATE --> DASHBOARD
    MODEL_LOAD --> DASHBOARD
    CONTEXT_SIZE --> DASHBOARD
```

### Distributed Tracing

Each request receives a trace ID that follows it through:
- Request validation
- Queue processing
- Agent selection
- Model processing
- Response generation

This enables end-to-end performance analysis and debugging.