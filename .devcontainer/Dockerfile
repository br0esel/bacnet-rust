# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.245.2/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version (use ubuntu-22.04 or ubuntu-18.04 on local arm64/Apple Silicon): ubuntu-22.04, ubuntu-20.04, ubuntu-18.04
FROM mcr.microsoft.com/devcontainers/base:jammy

ENV CROSS_CONTAINER_IN_CONTAINER=true
# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# RUN cargo install cross

 RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get -y install --no-install-recommends \
    #  build-essential \
     #liblua5.1-dev \
     #libcurl4-gnutls-dev \
    #  libmodbus-dev \
     #lua-socket \
    #  procps \
    #  pkg-config \
    #  cmake \
    #  gdb \
    docker.io \
    llvm-dev \
    libclang-dev \
    clang \
    # docker-compose \
    #  ninja-build \
    #  gcc-arm-linux-gnueabihf \
    #  g++-arm-linux-gnueabihf \
    #  libc6-dev-arm64-cross
     #gcc-10-arm-linux-gnueabihf \
     #g++-10-arm-linux-gnueabihf
    make

# RUN usermod -aG docker vscode
# RUN chown vscode /var/run/docker.sock