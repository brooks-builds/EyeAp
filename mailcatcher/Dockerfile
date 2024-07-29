FROM ruby:latest

EXPOSE 1080
EXPOSE 1025

RUN gem install mailcatcher

CMD ["mailcatcher", "--ip", "0.0.0.0", "-f"]
