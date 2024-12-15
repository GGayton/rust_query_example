# syntax=docker/dockerfile:1
FROM postgres:14.15-alpine3.21
COPY db/schema.sql /docker-entrypoint-initdb.d/