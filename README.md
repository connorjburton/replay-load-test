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

## Running evaluation

This tool has been written in 3 (to be 4) different lanaguages and/or runtimes. Each with it's own `Dockerfile`.

To run this you first need to generate some test data, see `scripts/generate-fake-data/README.md`, then copy the generated file in `scripts/generate-fake-data/data` into the root directory, renaming the file to `test-data.json`. You can change the `scripts/generate-fake-data/Dockerfile` to specify how many test records you would like to test against.

Once you have a `test-data.json` file in the root directory, you can run specify which language/runtime you would like to evaluate by changing the `dockerfile` property in `docker-compose.yaml`.

Finally, to start the evaluation, run `docker compose up --build`. You need to pass `--build` to ensure the `test-data.json` file is coped into the docker image at build time.

Each implementation will, by default, not output any logs for outgoing requests, however the Nginx server it sends requests to (see `scripts/bin-web-app`) will log out requests it recieves. This is to minimise inteference with collecting jitter metrics. At the end of each implementation it will log out the the average/min/max jitter values it collected.

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
- Rust 1.67 (Tokio)
    - Jitter
        - Average: 1.23ms
        - Minimum: 0.07ms
        - Maximum: 2.42ms

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
- Rust 1.67 (Tokio)
    - Jitter
        - Average: 1.27ms
        - Minimum: 0.17ms
        - Maximum: 110.74ms

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
- Rust 1.67 (Tokio)
    - Jitter
        - Average: 1.13ms
        - Minimum: 0.01ms
        - Maximum: 18.91ms

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
- Rust 1.67 (Tokio)
    - Jitter
        - Average: 1.08ms
        - Minimum: 0.01ms
        - Maximum: 11ms