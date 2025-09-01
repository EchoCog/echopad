# Mermaid Diagram Index

This document provides a comprehensive index of all Mermaid diagrams included in the Paddler technical architecture documentation.

## System Architecture Diagrams

### High-Level Architecture
- **[System Overview](./architecture/system-overview.md#high-level-architecture)** - Complete system architecture showing Balancer, Agent fleet, and client interactions
- **[Data Flow Overview](./architecture/system-overview.md#data-flow-overview)** - High-level request/response flow sequence diagram
- **[Deployment Patterns](./architecture/system-overview.md#deployment-patterns)** - Different deployment configurations

### Component Architecture
- **[Balancer Components](./architecture/core-components.md#architecture-overview)** - Internal structure of Balancer services
- **[Agent Components](./architecture/core-components.md#architecture-overview-1)** - Internal structure of Agent processes
- **[Communication Patterns](./architecture/core-components.md#websocket-communication)** - Inter-component communication flows
- **[State Management Flow](./architecture/core-components.md#state-synchronization)** - State reconciliation patterns

## Request Processing Flows

### End-to-End Processing
- **[Complete Request Lifecycle](./data-flow/request-processing.md#complete-request-lifecycle)** - Full request flow from client to response
- **[Request Reception](./data-flow/request-processing.md#1-request-reception)** - Request validation and queuing sequence
- **[Agent Processing](./data-flow/request-processing.md#3-agent-request-processing)** - Agent-side request handling
- **[Token Generation Flow](./data-flow/request-processing.md#4-token-generation-flow)** - State machine for token generation

### Request Types
- **[Completion Requests](./data-flow/request-processing.md#completion-requests)** - Text completion processing flow
- **[Embedding Requests](./data-flow/request-processing.md#embedding-requests)** - Embedding generation flow
- **[Error Handling](./data-flow/request-processing.md#error-handling-in-request-flow)** - Error propagation and recovery

### Performance Analysis
- **[Latency Breakdown](./data-flow/request-processing.md#latency-breakdown)** - Gantt chart of request processing timeline
- **[Monitoring Metrics](./data-flow/request-processing.md#key-metrics)** - Key performance indicators

## Deployment Architecture

### Deployment Patterns
- **[Single-Node Development](./deployment/README.md#single-node-development)** - Development environment setup
- **[Multi-Node Production](./deployment/README.md#multi-node-production)** - Production deployment architecture
- **[Kubernetes Deployment](./deployment/README.md#kubernetes-deployment)** - Cloud-native container deployment
- **[Network Architecture](./deployment/README.md#network-architecture)** - Network topology and security

### Scaling and High Availability
- **[Horizontal Scaling](./deployment/README.md#horizontal-scaling)** - Auto-scaling flow and triggers
- **[Multi-Region Deployment](./deployment/README.md#multi-region-deployment)** - Geographic distribution for HA
- **[Monitoring Strategy](./deployment/README.md#deployment-metrics)** - Infrastructure and application monitoring
- **[Cost Optimization](./deployment/README.md#cost-optimization-strategies)** - Resource optimization strategies

## State Management

### State Architecture
- **[State Management Overview](./state-management/README.md#state-management-overview)** - Distributed state layers and controllers
- **[State Types](./state-management/README.md#request-level-state)** - Request state machine
- **[Validation Pipeline](./state-management/README.md#state-validation)** - Configuration validation flow
- **[Reconciliation Loop](./state-management/README.md#reconciliation-loop)** - State synchronization sequence

### Consistency and Recovery
- **[Conflict Resolution](./state-management/README.md#conflict-resolution)** - State conflict handling
- **[Event-Driven Sync](./state-management/README.md#event-driven-synchronization)** - Event-based state updates
- **[Recovery Mechanisms](./state-management/README.md#recovery-mechanisms)** - Failure detection and recovery
- **[Network Partitions](./state-management/README.md#network-partitions)** - Partition handling sequence

### Monitoring and Debugging
- **[State Inspection](./state-management/README.md#state-inspection-tools)** - Debugging tools and inspection points
- **[State Divergence Alerts](./state-management/README.md#state-divergence-alerts)** - Automated divergence detection

## API Architecture

### API Overview
- **[API Structure](./api/README.md#api-overview)** - Complete API landscape
- **[Inference Endpoints](./api/README.md#inference-api-client-facing)** - Client-facing API structure
- **[Management Endpoints](./api/README.md#management-api-administrative)** - Administrative API structure
- **[Authentication Flow](./api/README.md#api-authentication)** - API authentication sequence

### Protocols and Communication
- **[HTTP Methods](./api/README.md#httprest-api)** - REST API structure
- **[WebSocket Communication](./api/README.md#websocket-api)** - Real-time communication flow
- **[Rate Limiting](./api/README.md#rate-limiting-strategy)** - Rate limiting decision flow
- **[Client Tiers](./api/README.md#quota-management)** - Quota management structure

### Performance and Monitoring
- **[Request Optimization](./api/README.md#request-optimization)** - Client and server optimizations
- **[API Versioning](./api/README.md#api-versioning)** - Version migration strategy
- **[API Metrics](./api/README.md#api-metrics)** - Performance monitoring dashboard
- **[Request Tracing](./api/README.md#request-tracing)** - Distributed tracing sequence

## Component Details

### Component Overview
- **[Component Architecture](./components/README.md#component-overview)** - Complete component landscape
- **[Dependency Graph](./components/README.md#dependency-graph)** - Component dependencies
- **[Startup Dependencies](./components/README.md#startup-dependencies)** - Initialization sequence

### Interaction Patterns
- **[Request Processing Flow](./components/README.md#request-processing-flow)** - Inter-component request sequence
- **[State Synchronization Flow](./components/README.md#state-synchronization-flow)** - State sync between components
- **[Service Interfaces](./components/README.md#internal-apis)** - Component interface structure

### Performance and Testing
- **[Performance Profiles](./components/README.md#component-performance-profiles)** - Component performance characteristics
- **[Error Handling](./components/README.md#component-level-error-handling)** - Error handling decision tree
- **[Circuit Breaker](./components/README.md#circuit-breaker-pattern)** - Circuit breaker state machine
- **[Testing Levels](./components/README.md#component-testing)** - Testing strategy overview

### Monitoring and Observability
- **[Component Metrics](./components/README.md#component-metrics)** - Multi-layer monitoring dashboard
- **[Distributed Tracing](./components/README.md#distributed-tracing)** - Component-level tracing sequence

## Diagram Usage Guidelines

### Viewing Diagrams
- All diagrams use Mermaid syntax and render automatically in GitHub
- For local viewing, use any Mermaid-compatible viewer or browser extension
- Diagrams are optimized for both light and dark themes

### Updating Diagrams
- Diagrams are stored as text in Markdown files for version control
- Use Mermaid Live Editor for development and testing
- Follow the existing diagram styling and color conventions
- Update this index when adding new diagrams

### Diagram Conventions
- **Blue** (#e3f2fd): External interfaces and client components
- **Orange** (#fff3e0): Management and configuration components  
- **Purple** (#f3e5f5): Processing and inference components
- **Green** (#e8f5e8): Successful states and positive flows
- **Red**: Error states and failure conditions