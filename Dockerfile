FROM gcr.io/distroless/static-debian12:nonroot

ARG TARGETOS
ARG TARGETARCH

WORKDIR /awsbck

# Copy the binary with correct permissions (requires buildx?)
COPY --chmod=0755 ./${TARGETOS}_${TARGETARCH}/awsbck .

USER nonroot:nonroot

# We use entrypoint to allow passing arguments to the binary using `CMD`
ENTRYPOINT ["/awsbck/awsbck"]
