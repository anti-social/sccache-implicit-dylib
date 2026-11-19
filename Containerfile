ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

ENV SCCACHE_RELEASES=https://github.com/mozilla/sccache/releases/download
ENV SCCACHE_VERSION=0.19.0
ENV SCCACHE_ARCHIVE_NAME=sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl
RUN \
  curl -fL ${SCCACHE_RELEASES}/v${SCCACHE_VERSION}/${SCCACHE_ARCHIVE_NAME}.tar.gz | \
  tar -xvz --strip-components=1 -C /usr/local/bin ${SCCACHE_ARCHIVE_NAME}/sccache

ENV RUSTC_WRAPPER=/usr/local/bin/sccache
