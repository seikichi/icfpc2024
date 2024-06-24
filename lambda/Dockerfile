FROM public.ecr.aws/lambda/nodejs:20

RUN dnf install -y gcc gcc-c++ openssl-devel

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install stable

WORKDIR ${LAMBDA_TASK_ROOT}
# COPY dashboard/prisma/schema.prisma ./prisma/schema.prisma
COPY lambda/package.json lambda/package-lock.json lambda/*.ts lambda/tsconfig.json ./
# RUN npm install && npm run prisma:generate && npm run build
RUN npm install && npm run build

# TODO: speedup
COPY solver/Cargo.lock solver/Cargo.toml ./
COPY solver/core/ core/
COPY solver/cli/ cli/
COPY solver/wasm/ wasm/
RUN cargo build --release

# CMD ["lambda.handler"]
