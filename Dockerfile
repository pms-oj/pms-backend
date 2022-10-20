FROM rust:bullseye

COPY . /opt/pms-backend

RUN apt upgrade -y && \
    apt install gcc git curl libpq-dev -y

RUN mkdir -p /app

WORKDIR /opt/pms-backend
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
RUN cp /opt/pms-backend/config.example.toml /app/config.toml
RUN cp /opt/pms-backend/log4rs.example.yaml /app/log4rs.yaml
RUN rm -rf /opt/pms-backend

EXPOSE 3031
EXPOSE 3030

ENTRYPOINT ["pms-backend"]