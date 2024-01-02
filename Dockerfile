FROM ubuntu:22.04
RUN apt-get update &&\
	apt-get install --fix-missing
RUN apt-get install build-essential mingw-w64 curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . /src/
WORKDIR /src/
RUN rustup target add x86_64-pc-windows-gnu
RUN cargo build --target x86_64-pc-windows-gnu --release
ENTRYPOINT [ "cp", "/src/target/x86_64-pc-windows-gnu/release/bfc-ng-runtime.exe", "/mount/" ]