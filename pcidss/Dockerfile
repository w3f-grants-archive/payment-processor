FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /pcidss

COPY . .

RUN cargo build --release --locked -p pcidss-oracle

FROM docker.io/debian:bullseye-slim

COPY --from=builder /pcidss/target/release/pcidss-oracle /usr/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /oracle oracle

# Create entrypoint script
RUN echo '#!/bin/bash\nset -xeu\n/usr/bin/pcidss-oracle $@' > /usr/bin/entrypoint.sh && \
    chmod +x /usr/bin/entrypoint.sh

# Switch to user oracle
USER oracle

EXPOSE 3030
ENTRYPOINT ["/usr/bin/entrypoint.sh"]
