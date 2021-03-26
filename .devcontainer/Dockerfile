FROM ianpurton/rust-devcontainer:latest AS base

FROM base AS development

COPY ./bash_history /home/vscode/.bash_history
RUN sudo chown vscode /home/vscode/.bash_history
