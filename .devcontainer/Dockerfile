FROM ianpurton/rust-devcontainer:latest AS base

FROM base AS development

COPY ./bash_history /home/vscode/.bash_history
RUN sudo chown vscode /home/vscode/.bash_history

RUN sudo apt update \
    && sudo apt install -y protobuf-compiler \
    && sudo wget https://github.com/improbable-eng/grpc-web/releases/download/v0.14.0/grpcwebproxy-v0.14.0-linux-x86_64.zip  \
    && sudo unzip grpc* \
    && sudo mv dist/grpcwebproxy-v0.14.0-linux-x86_64 /usr/bin/grpcwebproxy \
    && sudo rm *.zip 
