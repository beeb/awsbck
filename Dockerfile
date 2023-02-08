FROM debian:bullseye-slim AS builder

ARG TARGETOS
ARG TARGETARCH

WORKDIR /awsbck

# Copy binary and adjust permissions
COPY ./${TARGETOS}_${TARGETARCH}/awsbck .
RUN chmod +x awsbck

FROM gcr.io/distroless/static:nonroot

WORKDIR /awsbck

# Copy the binary with correct permissions
COPY --from=builder /awsbck/awsbck .

USER nonroot:nonroot

# We use entrypoint to allow passing arguments to the binary using `CMD`
ENTRYPOINT ["/awsbck/awsbck"]
