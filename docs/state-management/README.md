# State Management Architecture

This document details how Paddler manages state across distributed components and maintains consistency in a dynamic system.

## State Management Overview

Paddler implements a distributed state management system that ensures consistency between the desired system configuration and the actual runtime state across all components.

```mermaid
graph TB
    subgraph "State Layers"
        DESIRED[Desired State<br/>Configuration Intent]
        APPLICABLE[Applicable State<br/>Resolved Configuration]
        CURRENT[Current State<br/>Runtime Reality]
    end
    
    subgraph "State Controllers"
        RECONCILER[Reconciliation Engine]
        STATE_DB[State Database]
        SYNC_SVC[Synchronization Service]
    end
    
    subgraph "State Sources"
        CONFIG[Configuration Files]
        API[Management API]
        AUTO[Auto-discovery]
    end
    
    CONFIG --> DESIRED
    API --> DESIRED
    AUTO --> CURRENT
    
    DESIRED --> RECONCILER
    RECONCILER --> APPLICABLE
    APPLICABLE --> CURRENT
    
    RECONCILER --> STATE_DB
    STATE_DB --> SYNC_SVC
    SYNC_SVC --> RECONCILER
    
    style DESIRED fill:#e3f2fd
    style APPLICABLE fill:#fff3e0
    style CURRENT fill:#e8f5e8
```

## State Types and Scope

### System-Level State

**Balancer State**:
- Registered agents and their health
- Available models and metadata
- System configuration and policies
- Performance metrics and history

**Agent State**:
- Slot availability and status
- Loaded models and their versions
- Resource utilization metrics
- Connection status to balancer

### Request-Level State

**Active Requests**:
- Request queue contents
- In-progress inference tasks
- Streaming response sessions
- Request routing decisions

```mermaid
stateDiagram-v2
    [*] --> Queued
    Queued --> Routed: Agent Selected
    Routed --> Processing: Sent to Agent
    Processing --> Streaming: Generation Started
    Streaming --> Completed: Generation Done
    Processing --> Failed: Error Occurred
    Routed --> Failed: Agent Unavailable
    Completed --> [*]
    Failed --> [*]
    
    note right of Processing
        Request state is tracked
        throughout the pipeline
    end note
```

## Desired State Specification

### Configuration Schema

Paddler uses a declarative configuration model where operators specify the desired end state:

```yaml
# Example desired state configuration
agents:
  - id: "agent-1"
    desired_model:
      source: "huggingface"
      model_id: "microsoft/DialoGPT-medium"
      revision: "main"
    slots: 4
    max_context_length: 2048
    
  - id: "agent-2"  
    desired_model:
      source: "local"
      path: "/models/llama-7b.gguf"
    slots: 2
    max_context_length: 4096

models:
  - id: "default-chat"
    chat_template: |
      {% for message in messages %}
      {{ message.role }}: {{ message.content }}
      {% endfor %}
    parameters:
      temperature: 0.7
      top_p: 0.9
      max_tokens: 1024
```

### State Validation

```mermaid
graph LR
    subgraph "Validation Pipeline"
        INPUT[Configuration Input]
        SCHEMA[Schema Validation]
        SEMANTIC[Semantic Validation]
        RESOURCE[Resource Validation]
        APPROVED[Approved State]
    end
    
    INPUT --> SCHEMA
    SCHEMA -->|Valid| SEMANTIC
    SCHEMA -->|Invalid| ERROR1[Schema Error]
    
    SEMANTIC -->|Valid| RESOURCE
    SEMANTIC -->|Invalid| ERROR2[Semantic Error]
    
    RESOURCE -->|Valid| APPROVED
    RESOURCE -->|Invalid| ERROR3[Resource Error]
    
    ERROR1 --> REJECT[Reject Configuration]
    ERROR2 --> REJECT
    ERROR3 --> REJECT
```

## State Reconciliation Process

### Reconciliation Loop

```mermaid
sequenceDiagram
    participant Config as Configuration
    participant Reconciler as Reconciliation Engine
    participant StateDB as State Database
    participant Agent as Agent
    
    loop Continuous Reconciliation
        Reconciler->>StateDB: Get Current State
        Reconciler->>Config: Get Desired State
        Reconciler->>Reconciler: Calculate Diff
        
        alt Changes Needed
            Reconciler->>Agent: Apply Changes
            Agent->>Reconciler: Report Status
            Reconciler->>StateDB: Update State
        else No Changes
            Note over Reconciler: Wait for next cycle
        end
    end
```

### Conflict Resolution

When multiple sources attempt to modify state simultaneously:

```mermaid
graph TD
    CONFLICT[State Conflict Detected] --> PRIORITY[Priority Resolution]
    
    subgraph "Priority Order"
        API_UPDATE[API Updates<br/>Priority: 1]
        CONFIG_FILE[Configuration Files<br/>Priority: 2]  
        AUTO_DISCOVERY[Auto-discovery<br/>Priority: 3]
    end
    
    PRIORITY --> API_UPDATE
    PRIORITY --> CONFIG_FILE
    PRIORITY --> AUTO_DISCOVERY
    
    API_UPDATE --> RESOLVE[Apply Highest Priority]
    CONFIG_FILE --> RESOLVE
    AUTO_DISCOVERY --> RESOLVE
    
    RESOLVE --> NOTIFY[Notify Conflicting Sources]
```

## State Synchronization Patterns

### Event-Driven Synchronization

```mermaid
graph LR
    subgraph "Event Publishers"
        AGENT_EVENTS[Agent Events]
        CONFIG_EVENTS[Config Events]
        HEALTH_EVENTS[Health Events]
    end
    
    subgraph "Event Bus"
        BUS[Event Bus<br/>WebSocket/Message Queue]
    end
    
    subgraph "Event Subscribers"
        RECONCILER[Reconciler]
        MONITOR[Health Monitor]
        METRICS[Metrics Collector]
    end
    
    AGENT_EVENTS --> BUS
    CONFIG_EVENTS --> BUS
    HEALTH_EVENTS --> BUS
    
    BUS --> RECONCILER
    BUS --> MONITOR
    BUS --> METRICS
```

### Periodic Synchronization

Complements event-driven updates with periodic full synchronization:

```mermaid
gantt
    title State Synchronization Timeline
    dateFormat X
    axisFormat %s
    
    section Health Checks
    Agent Health       :0, 30
    Model Status       :30, 60
    Resource Check     :60, 90
    
    section State Sync
    Incremental Sync   :active, 0, 10
    Incremental Sync   :active, 10, 20
    Full Sync         :crit, 60, 70
    Incremental Sync   :active, 70, 80
```

## State Persistence and Recovery

### State Database Schema

```mermaid
erDiagram
    AGENTS {
        string id PK
        string status
        timestamp last_seen
        json configuration
        json capabilities
    }
    
    MODELS {
        string id PK
        string source_type
        string source_location
        json metadata
        timestamp loaded_at
    }
    
    SLOTS {
        string id PK
        string agent_id FK
        string status
        string model_id FK
        int context_length
        timestamp last_request
    }
    
    REQUESTS {
        string id PK
        string slot_id FK
        string status
        timestamp created_at
        timestamp completed_at
        json parameters
    }
    
    AGENTS ||--o{ SLOTS : has
    MODELS ||--o{ SLOTS : loaded_in
    SLOTS ||--o{ REQUESTS : processes
```

### Recovery Mechanisms

```mermaid
graph TD
    FAILURE[Component Failure] --> DETECT[Failure Detection]
    DETECT --> ASSESS[Assess Impact]
    
    ASSESS --> AGENT_FAIL{Agent Failure?}
    ASSESS --> BALANCER_FAIL{Balancer Failure?}
    
    AGENT_FAIL -->|Yes| REMOVE_AGENT[Remove from Pool]
    REMOVE_AGENT --> REDISTRIBUTE[Redistribute Requests]
    REDISTRIBUTE --> RECOVERY_COMPLETE[Recovery Complete]
    
    BALANCER_FAIL -->|Yes| FAILOVER[Failover to Backup]
    FAILOVER --> RESTORE_STATE[Restore State from DB]
    RESTORE_STATE --> RECONNECT[Reconnect Agents]
    RECONNECT --> RECOVERY_COMPLETE
    
    AGENT_FAIL -->|No| RECOVERY_COMPLETE
    BALANCER_FAIL -->|No| RECOVERY_COMPLETE
```

## Distributed State Challenges

### Network Partitions

Handling network splits between balancer and agents:

```mermaid
sequenceDiagram
    participant Balancer
    participant Agent1
    participant Agent2
    participant StateDB
    
    Note over Balancer,Agent2: Network Partition Occurs
    
    Balancer->>Agent1: Health Check
    Agent1->>Balancer: OK Response
    
    Balancer->>Agent2: Health Check
    Note over Agent2: No Response (Partition)
    
    Balancer->>StateDB: Mark Agent2 Unhealthy
    Balancer->>Agent1: Increase Load
    
    Note over Balancer,Agent2: Partition Heals
    
    Agent2->>Balancer: Reconnect
    Balancer->>Agent2: State Synchronization
    Agent2->>Balancer: Current State Report
    Balancer->>StateDB: Update Agent2 Status
```

### Eventual Consistency

```mermaid
graph LR
    subgraph "Consistency Model"
        STRONG[Strong Consistency<br/>Configuration Changes]
        EVENTUAL[Eventual Consistency<br/>Status Updates]
        WEAK[Weak Consistency<br/>Metrics/Logs]
    end
    
    subgraph "Trade-offs"
        LATENCY[Higher Latency]
        AVAILABILITY[Higher Availability]  
        PARTITION[Partition Tolerance]
    end
    
    STRONG --> LATENCY
    EVENTUAL --> AVAILABILITY
    WEAK --> PARTITION
```

## State Monitoring and Debugging

### State Inspection Tools

```mermaid
graph LR
    subgraph "Debugging Tools"
        STATE_DUMP[State Dump API]
        DIFF_TOOL[State Diff Viewer]
        RECONCILE_LOG[Reconciliation Logs]
        HEALTH_DASH[Health Dashboard]
    end
    
    subgraph "Inspection Points"
        DESIRED_STATE[Desired State]
        APPLICABLE_STATE[Applicable State]
        CURRENT_STATE[Current State]
        AGENT_STATE[Agent-Reported State]
    end
    
    STATE_DUMP --> DESIRED_STATE
    STATE_DUMP --> APPLICABLE_STATE
    STATE_DUMP --> CURRENT_STATE
    STATE_DUMP --> AGENT_STATE
    
    DIFF_TOOL --> DESIRED_STATE
    DIFF_TOOL --> CURRENT_STATE
```

### State Divergence Alerts

```mermaid
graph TB
    MONITOR[State Monitor] --> CHECK{State Consistent?}
    
    CHECK -->|Yes| CONTINUE[Continue Monitoring]
    CHECK -->|No| ALERT[Generate Alert]
    
    ALERT --> CLASSIFY[Classify Divergence]
    
    CLASSIFY --> CONFIG_DRIFT[Configuration Drift]
    CLASSIFY --> NETWORK_ISSUE[Network Issues]
    CLASSIFY --> RESOURCE_CONSTRAINT[Resource Constraints]
    CLASSIFY --> SOFTWARE_BUG[Software Bug]
    
    CONFIG_DRIFT --> AUTO_RECONCILE[Auto-reconcile]
    NETWORK_ISSUE --> RETRY[Retry Sync]
    RESOURCE_CONSTRAINT --> SCALE_UP[Scale Resources]
    SOFTWARE_BUG --> MANUAL_INTERVENTION[Manual Investigation]
    
    AUTO_RECONCILE --> CONTINUE
    RETRY --> CONTINUE
    SCALE_UP --> CONTINUE
```

## Best Practices

### State Management Guidelines

1. **Immutable State**: Treat state changes as creating new versions
2. **Event Sourcing**: Maintain history of state changes for debugging
3. **Graceful Degradation**: System continues with partial state loss
4. **Conflict Avoidance**: Design to minimize state conflicts
5. **Observability**: Comprehensive logging and monitoring of state changes

### Performance Optimization

```mermaid
graph LR
    subgraph "Optimization Strategies"
        CACHING[State Caching]
        BATCHING[Batch Updates]
        COMPRESSION[Delta Compression]
        INDEXING[State Indexing]
    end
    
    subgraph "Benefits"
        REDUCED_LATENCY[Reduced Latency]
        LOWER_BANDWIDTH[Lower Bandwidth]
        FASTER_QUERIES[Faster Queries]
        BETTER_SCALE[Better Scalability]
    end
    
    CACHING --> REDUCED_LATENCY
    BATCHING --> LOWER_BANDWIDTH
    COMPRESSION --> LOWER_BANDWIDTH
    INDEXING --> FASTER_QUERIES
    
    REDUCED_LATENCY --> BETTER_SCALE
    LOWER_BANDWIDTH --> BETTER_SCALE
    FASTER_QUERIES --> BETTER_SCALE
```

### Testing State Management

1. **Chaos Testing**: Simulate network partitions and failures
2. **State Consistency Tests**: Verify state converges after disruptions
3. **Performance Testing**: Measure state sync latency under load
4. **Recovery Testing**: Validate recovery from various failure scenarios