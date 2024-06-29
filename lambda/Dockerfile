FROM public.ecr.aws/lambda/nodejs:20

RUN dnf install -y gcc gcc-c++ openssl-devel

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install stable

WORKDIR ${LAMBDA_TASK_ROOT}
COPY courses/ ./course
COPY dashboard/prisma/schema.prisma ./prisma/schema.prisma
COPY lambda/package.json lambda/package-lock.json lambda/*.ts lambda/tsconfig.json ./
# RUN npm install && npm run prisma:generate && npm run build
RUN npm install && npm run prisma:generate && npm run build

# Cache Rust dependencies with Docker build
# see: https://stackoverflow.com/questions/58473606
COPY solver/Cargo.lock solver/Cargo.toml ./
COPY solver/wasm/Cargo.toml ./wasm/
RUN mkdir -p ./wasm/src && echo "fn f() {}" > ./wasm/src/lib.rs
COPY solver/base/Cargo.toml ./base/
RUN mkdir -p ./base/src && echo "fn f() {}" > ./base/src/lib.rs

# solvers
COPY solver/lambdaman/Cargo.toml ./wasm/
RUN mkdir -p ./lambdaman/src && echo "fn main() { println!(\"dummy\"); }" > ./lambdaman/src/main.rs
COPY solver/spaceship/Cargo.toml ./wasm/
RUN mkdir -p ./spaceship/src && echo "fn main() { println!(\"dummy\"); }" > ./spaceship/src/main.rs
RUN cargo build --release

# copy whole source
COPY solver/wasm ./wasm
COPY solver/base ./bae

# solvers
COPY solver/lambdaman ./lambdaman
COPY solver/spaceship ./spaceship

# solvers
RUN touch ./wasm/src/lib.rs ./base/src/lib.rs ./lambdaman/src/main.rs ./spaceship/src/main.rs
RUN cargo build --release

# CMD ["lambda.handler"]
