FROM debian:latest AS generate
WORKDIR /gen
ENV PATH="/root/.local/bin:${PATH}"
ENV RUST_BACKTRACE=full
RUN bash
RUN apt-get update -y && apt-get install curl jq -y
RUN curl --proto "=https" --tlsv1.2 -sSL https://getsynth.com/install | sh -s -- --ci
COPY . .
RUN synth generate . --collection logs --size 1000 --to json:./data/output.$(date +"%Y-%m-%d_%H-%M-%S").json

FROM scratch as generate-export
COPY --from=generate /gen/data /