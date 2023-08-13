FROM nginx:latest

COPY server.conf /etc/nginx/conf.d/server.conf

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]