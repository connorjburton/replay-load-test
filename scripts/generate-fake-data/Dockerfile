FROM debian:latest AS generate
WORKDIR /app
ENV PATH="/root/.local/bin:${PATH}"
ENV RUST_BACKTRACE=full
RUN bash
RUN apt-get update -y && apt-get install curl jq -y
RUN curl --proto "=https" --tlsv1.2 -sSL https://getsynth.com/install | sh -s -- --ci
COPY . .
# To generate unique file names you can replace `test-data.json` with this
# output.$(date +"%Y-%m-%d_%H-%M-%S").json
RUN synth generate . --collection logs --size 100 --to json:./data/test-data.json
