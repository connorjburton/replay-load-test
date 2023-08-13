FROM denoland/deno:1.33.2

USER deno

WORKDIR /app

COPY . .

CMD ["run", "--allow-read", "--allow-net=bin-web-app:8080", "index.ts"]