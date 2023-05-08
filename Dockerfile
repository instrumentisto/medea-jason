#
# Dockerfile of instrumentisto/medea-demo:edge Docker image.
#


#
# Stage 'dist' creates project distribution.
#

# https://hub.docker.com/_/rust
# ARG rust_ver=latest
# FROM ghcr.io/instrumentisto/rust:${rust_ver} AS dist
# ARG debug=no

# # RUN cargo install wasm-pack \
# #  && rustup target add wasm32-unknown-unknown

# COPY / /src/

# # RUN cd /src/ \
# #  && make cargo.build.jason platform=web debug=${debug} dockerized=no

# # RUN cd /src/ \
# #  && npm run build --prefix=./e2e-demo



#
# Stage 'runtime' creates final Docker image to use in runtime.
#

# https://hub.docker.com/_/nginx
FROM nginx:stable-alpine AS runtime

COPY demo/chart/medea-demo/conf/nginx.vh.conf \
     /etc/nginx/conf.d/default.conf
COPY demo/chart/medea-demo/conf/fullchain.pem \
     /etc/nginx/fullchain.pem
COPY demo/chart/medea-demo/conf/privkey.pem \
     /etc/nginx/privkey.pem

COPY /e2e-demo/dist/*.html /app/
COPY /e2e-demo/dist/*.js /app/js/
COPY /e2e-demo/dist/*.wasm /app/js/

WORKDIR /app

