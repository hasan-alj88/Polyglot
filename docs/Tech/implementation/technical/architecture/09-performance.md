## Performance Considerations

### Compilation Speed (NFR-P1)

**Target:** <1s compilation for 1000-line `.pg` files

**Strategies:**
- Efficient lexer (single-pass)
- Parser uses zero-copy where possible
- IR generation avoids unnecessary clones
- Benchmark compilation in CI

### Pipeline Execution Latency (NFR-P2)

**Target:** <2s from trigger to execution start

**Strategies:**
- PostgreSQL connection pooling (reuse connections)
- Redis pipelining for queue operations
- Minimize database roundtrips
- Index optimization on `pipelines(activated)` and `pipeline_instances(status)`

### Type Conversion Overhead (NFR-P3)

**Target:** <10ms for typical data sizes (<1MB)

**Strategies:**
- JSON serialization (serde_json is highly optimized)
- Streaming deserialization for large payloads
- Benchmark runtime wrapper performance
- Future: Upgrade to bincode for production if needed

### Queue Throughput (NFR-P4)

**Target:** 100+ instances/second

**Strategies:**
- Redis `RPUSH`/`BLPOP` are O(1) operations
- Batch queue operations where possible
- Monitor queue depth and lag

### Database Query Performance (NFR-P5)

**Target:** <100ms for registry queries, <500ms for logs

**Strategies:**
- Indexes on frequently queried columns
- Use `EXPLAIN ANALYZE` to optimize slow queries
- Connection pooling (default: 10 connections)
- Prepared statements via SQLx (cache query plans)

---

