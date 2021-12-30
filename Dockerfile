FROM rust:1.57-slim AS build

RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https \
  build-essential curl wget

ENV PERSEUS_VERSION=0.3.2 \
  PERSEUS_SIZE_OPT_VERSION=0.1.7 \
  ESBUILD_VERSION=0.14.7 \
  BINARYEN_VERSION=104

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

COPY . contextswitch-web

WORKDIR /app/contextswitch-web

RUN cargo install perseus-cli --version $PERSEUS_VERSION
RUN perseus clean && perseus prep
RUN perseus tinker \
  && cat .perseus/Cargo.toml \
  && cat ./src/lib.rs

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

RUN perseus deploy

WORKDIR /app

RUN curl -O https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar xf esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && ./package/bin/esbuild --version

RUN ./package/bin/esbuild ./contextswitch-web/pkg/dist/pkg/perseus_engine.js --minify --target=es6 --outfile=./contextswitch-web/pkg/dist/pkg/perseus_engine.js --allow-overwrite \
  && ls -lha ./contextswitch-web/pkg/dist/pkg

RUN wget -nv https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar xf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt --version

RUN ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt -Os ./contextswitch-web/pkg/dist/pkg/perseus_engine_bg.wasm -o ./contextswitch-web/pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./contextswitch-web/pkg/dist/pkg

FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/contextswitch-web/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
