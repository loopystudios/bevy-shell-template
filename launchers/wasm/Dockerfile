FROM nginx as webserver
COPY ./dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf
COPY ./Docker.nginx.conf /etc/nginx/conf.d/Docker.conf

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]
