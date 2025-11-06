FROM eclipse-temurin:21-jre-alpine-3.21

RUN mkdir -p /opt/httploadgenerator
COPY httploadgenerator.jar /opt/httploadgenerator/

ENV RUNMODE loadgenerator

EXPOSE 24225

CMD java -jar /opt/httploadgenerator/httploadgenerator.jar loadgenerator

