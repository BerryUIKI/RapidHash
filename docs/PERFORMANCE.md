# Performance Policy

## Principles

Performance work must preserve correctness, cancellation, bounded resources, and
system responsiveness. Claims require reproducible benchmarks and must identify
hardware, operating system, file system, storage, data set, algorithms, build
profile, and competing workload.

## Metrics

- wall-clock throughput for large sequential files;
- files per second for small-file workloads;
- time to first visible result;
- cancellation latency;
- peak resident memory;
- CPU utilization and efficiency;
- UI frame responsiveness; and
- manifest parse and verification time.

## Reference Workloads

- one empty file;
- 10,000 small files with mixed path lengths;
- one file larger than available memory;
- a mixed directory tree;
- simultaneous algorithms in one read pass;
- a manifest with valid and invalid records; and
- a throttled or high-latency file system.

Generated benchmark data must not be committed if it is large or contains local
paths.

## Resource Budgets

Buffers, workers, open handles, discovery queues, and UI events remain bounded.
Defaults should adapt conservatively to logical CPU count and workload. Storage
throughput should not be harmed by excessive file-level concurrency.

## Optimization Process

1. Add or select a representative benchmark.
2. Record a baseline.
3. Profile the limiting resource.
4. Make the smallest justified change.
5. Verify all correctness and security tests.
6. Report results with uncertainty and tradeoffs.

Microbenchmarks alone do not justify architecture changes.
