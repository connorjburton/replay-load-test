FROM denoland/deno:1.33.2

USER deno

WORKDIR /app

COPY . .

CMD ["run", "--allow-read", "--allow-write", "index.ts"]