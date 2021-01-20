FROM ubuntu:19.04

WORKDIR /usr/src/csml

COPY ./target/release/csml_server server

RUN chmod 755 server

RUN groupadd -r csml && useradd -r -g csml csml
USER csml

EXPOSE 5000

CMD ./server
