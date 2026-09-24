FROM ruby:3.2-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Gemfile* ./
RUN bundle install

COPY . .

EXPOSE 8003

CMD ["bundle", "exec", "puma", "-p", "8003", "-b", "tcp://0.0.0.0:8003"]
