FROM alpine:3.5

RUN addgroup -S app && adduser -S -g app app 
ENV HOME=/home/app

COPY target/release/service1 /usr/local/bin/
COPY Rocket.toml /usr/local/bin/

EXPOSE 8002

USER app
WORKDIR $HOME
ENTRYPOINT ["/usr/local/bin/service2"]
