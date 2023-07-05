FROM rust:1.70-alpine3.18 AS builderBackend

WORKDIR /work

COPY migrations/ ./migrations/
COPY rauthy-common/ ./rauthy-common/
COPY rauthy-handlers/ ./rauthy-handlers/
COPY rauthy-main/ ./rauthy-main/
COPY rauthy-models/ ./rauthy-models/
COPY rauthy-service/ ./rauthy-service/
COPY static ./static/
COPY templates ./templates/
COPY Cargo.* ./

RUN mkdir data

FROM scratch

USER 10001:10001

WORKDIR /app

#COPY --chown=10001:10001 --from=builderBackend /work/target/release/rauthy .
COPY --chown=10001:10001 /out/rauthy .
COPY --chown=10001:10001 --from=builderBackend /work/data ./data

COPY --chown=10001:10001 tls/ ./tls/
COPY --chown=10001:10001 rauthy.deploy.cfg ./rauthy.cfg

CMD ["/app/rauthy"]
