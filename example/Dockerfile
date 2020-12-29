FROM ubuntu:latest

ENV EXAMPLE_VALUE_ONE=John
ENV EXAMPLE_VALUE_TWO=Doe
ENV EXAMPLE_VALUE_THREE=Alice

RUN apt-get update && apt-get install -y curl

RUN curl -L https://github.com/kongand/serve-env/releases/download/v1.0.0-alpha.4/serve-env > serve-env

RUN chmod +x serve-env

EXPOSE 3030

CMD ["./serve-env", "-k", "EXAMPLE_VALUE_"]