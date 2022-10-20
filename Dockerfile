FROM oraclelinux:9-slim

COPY . /opt/pms-backend

RUN microdnf upgrade -y && \
    microdnf install gcc git curl libpq-devel -y

RUN mkdir -p /opt/rust /app

WORKDIR /opt/rust
RUN curl https://sh.rustup.rs -s >> rustup.sh
RUN chmod 755 /opt/rust/rustup.sh
RUN ./rustup.sh -y

ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /opt/pms-backend
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
RUN cp /opt/pms-backend/config.example.toml /app/config.toml
RUN cp /opt/pms-backend/log4rs.example.yaml /app/log4rs.yaml
RUN rm -rf /opt/pms-backend /opt/rust

EXPOSE 3031
EXPOSE 3030

ENTRYPOINT ["pms-backend"]