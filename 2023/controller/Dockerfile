ARG CROSS_PLATFORM
ARG CROSS_VERSION
FROM ghcr.io/cross-rs/$CROSS_PLATFORM:$CROSS_VERSION

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libssl-dev:armhf

ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_LIBDIR_$CROSS_PLATFORM=/usr/lib/arm-linux-gnueabihf/pkgconfig
