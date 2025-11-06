FROM eclipse-temurin:21-jre-alpine-3.21

RUN mkdir -p /opt/httploadgenerator
COPY httploadgenerator.jar /opt/httploadgenerator/

EXPOSE 8080

WORKDIR /loadgenerator

