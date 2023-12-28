FROM node:19.5.0-alpine3.17 as FrontendBuilder
WORKDIR money_app
COPY /frontend ./frontend
WORKDIR /money_app/frontend
RUN npm install &&\
    npm run build

FROM bitnami/git:2.43.0-debian-11-r4 as DependencyDownloader
WORKDIR /fancy_surreal
RUN git clone --depth 1 --branch 0.1.1 https://github.com/xilefmusics/fancy_surreal.git .

FROM rust:1.74.1-slim-buster as BackendBuilder
COPY --from=DependencyDownloader /fancy_surreal /fancy_surreal
WORKDIR /money_app
COPY /backend ./backend
WORKDIR /money_app/backend
RUN cargo build --release

FROM ubuntu:22.04

COPY --from=FrontendBuilder money_app/frontend/build /app/static
COPY --from=BackendBuilder money_app/backend/target/release/backend /app/worship_viewer
#
ENV PORT="8000" \
    DB_HOST="db" \
    DB_PORT="8000" \
    DB_USER="root" \
    DB_PASSWORD="root" \
    DB_NAMESPACE="test" \
    DB_DATABASE="test" \
    STATIC_DIR="/app/static" \
    ATTACHMENT_DIR="/app/attachments"

VOLUME "/app/attachments"

ENTRYPOINT ["/app/worship_viewer"]
