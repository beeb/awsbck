FROM gcr.io/distroless/static

ARG TARGETOS
ARG TARGETARCH

WORKDIR /awsbck

# Copy the binary
COPY ./${TARGETOS}_${TARGETARCH}/awsbck .

# We use entrypoint to allow passing arguments to the binary using `CMD`
ENTRYPOINT ["/awsbck/awsbck"]
