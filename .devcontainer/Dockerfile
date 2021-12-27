FROM ianpurton/rust-fullstack-devcontainer:latest

# Add our aliases and ps1
COPY *.bash ./
RUN cat aliases.bash >> ~/.bashrc && cat ps1.bash >> ~/.bashrc && sudo rm *.bash

# Give us access to docker in docker
RUN echo 'sudo chmod 777 /var/run/docker.sock' >> ~/.bashrc

RUN sudo apt update \
    && sudo apt install -y protobuf-compiler unzip \
    && sudo curl -OL https://github.com/improbable-eng/grpc-web/releases/download/v0.14.0/grpcwebproxy-v0.14.0-linux-x86_64.zip  \
    && sudo unzip grpc* \
    && sudo mv dist/grpcwebproxy-v0.14.0-linux-x86_64 /usr/bin/grpcwebproxy \
    && sudo rm *.zip 
