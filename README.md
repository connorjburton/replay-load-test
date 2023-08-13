# Philosophy

- **Use the appropriate tools.** Choice of languages or dependencies should be chosen to achieve the goals of the project, not based on personal preference.
- **Stability.**
- **Performance.**
- **Generic.** Input & Output should be generic and as vast as possible. E.g input should be CSV, JSON, TOML etc
- **Respect users resources.** Should use the minimum resources possible. Given the user the ability to customise this where appropriate.
- **Leveled logging.**

# Appropriate Tools

The language that is used should have perform well on the following metrics, which should be measured whilst testing each language, in the fairest way possible.

- **Jitter.** The smallest jitter between the set delay and the execution of the function. E.g setting a delay of 100ms, executing the function 100ms after setting that delay is perfect.
- **High volume.** Being able to handle sending a huge volume of requests in a short time frame.
- **Resource usage.** Using a smaller amount of resources to run the application is better
- **Concurrency.** Being able to perform workloads without affecting the timing of the application is important for progressive scheduling.

## Evaulation Results

All tests ran with no logging other than start/end.

100 Requests over 5 minutes

- Deno 1.39
    - Jitter
        - Average: 0.88ms
        - Minimum: -1ms
        - Maximum: 6ms
- Node.js 20.5.1
    - Jitter
        - Average: 0.12ms
        - Minimum: -3ms
        - Maximum: 16ms

1000 Requests over 5 minutes

- Deno 1.39
    - Jitter
        - Average: 0.72ms
        - Minimum: 0ms
        - Maximum: 7ms
- Node.js 20.5.1
    - Jitter
        - Average: -2.362ms
        - Minimum: -7ms
        - Maximum: 8ms

10000 Requests over 5 minutes

- Deno 1.39
    - Jitter
        - Average: -2.3642ms
        - Minimum: -7ms
        - Maximum: 35ms
- Node.js 20.5.1
    - Jitter
        - Average: 0.5965ms
        - Minimum: -2ms
        - Maximum: 26ms

100000 Requests over 5 minutes

- Deno 1.39
    - Jitter
        - Average: 0.37269ms
        - Minimum: -3ms
        - Maximum: 436ms
- Node.js 20.5.1
    - Jitter
        - Average: 0.14702ms
        - Minimum: -4ms
        - Maximum: 372ms