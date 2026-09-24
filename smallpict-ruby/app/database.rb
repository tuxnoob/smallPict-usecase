require 'sequel'
require_relative 'config'

DB = Sequel.connect(AppConfig.database_url, max_connections: 10)

# Create table public.images if not exists
DB.run <<-SQL
  CREATE TABLE IF NOT EXISTS public.images (
    id VARCHAR(50) PRIMARY KEY,
    filename VARCHAR(255),
    url TEXT,
    size BIGINT,
    size_origin BIGINT,
    mime_type VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );
SQL

module DatabaseHealth
  def self.healthy?
    DB.test_connection
    true
  rescue StandardError
    false
  end
end
