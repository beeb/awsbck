FROM gcr.io/distroless/static:nonroot

ARG TARGETOS
ARG TARGETARCH

WORKDIR /awsbck

# Copy the binary
COPY ./${TARGETOS}_${TARGETARCH}/awsbck .

USER nonroot:nonroot

# We use entrypoint to allow passing arguments to the binary using `CMD`
ENTRYPOINT ["/awsbck/awsbck"]
