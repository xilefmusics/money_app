FROM golang:1.20rc3-alpine3.17 as Builder
WORKDIR /go/src
COPY ./ ./
RUN go get . && CGO_ENABLED=0 go install

FROM scratch
COPY --from=Builder /go/bin/money-app /

VOLUME "/data"
EXPOSE 8080

ENV PORT="8080"
ENV DATA_PATH="../data/"

ENTRYPOINT [ "/money-app" ]
