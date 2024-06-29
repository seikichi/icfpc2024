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
COPY solver/core/Cargo.toml ./core/
RUN mkdir -p ./core/src && echo "fn f() {}" > ./core/src/lib.rs
COPY solver/cli/Cargo.toml ./cli/
RUN mkdir -p ./cli/src && echo "fn main() { println!(\"dummy\"); }" > ./cli/src/main.rs

# solvers
COPY solver/lambdaman/Cargo.toml ./wasm/
RUN mkdir -p ./lambdaman/src && echo "fn main() { println!(\"dummy\"); }" > ./lambdaman/src/main.rs
COPY solver/spaceship/Cargo.toml ./wasm/
RUN mkdir -p ./spaceship/src && echo "fn main() { println!(\"dummy\"); }" > ./spaceship/src/main.rs
RUN cargo build --release

# copy whole source
COPY solver/wasm ./wasm
COPY solver/core ./core
COPY solver/cli ./cli

# solvers
COPY solver/lambdaman ./lambdaman
COPY solver/spaceship ./spaceship

# solvers
RUN touch ./wasm/src/lib.rs ./core/src/lib.rs ./cli/src/main.rs ./lambdaman/src/main.rs ./spaceship/src/main.rs
RUN cargo build --release

# CMD ["lambda.handler"]
