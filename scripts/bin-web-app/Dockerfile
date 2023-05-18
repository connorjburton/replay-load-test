FROM denoland/deno:1.33.4

EXPOSE 8080

USER deno

COPY index.ts .

ENTRYPOINT deno run --allow-net index.ts