require 'dotenv/load'

module AppConfig
  def self.app_name
    ENV.fetch('APP_NAME', 'SmallPict Ruby Service')
  end

  def self.port
    ENV.fetch('APP_PORT', 8003).to_i
  end

  def self.host
    ENV.fetch('APP_HOST', '0.0.0.0')
  end

  def self.database_url
    host = ENV.fetch('DB_HOST', 'localhost')
    port = ENV.fetch('DB_PORT', 5432)
    user = ENV.fetch('DB_USER', 'postgres')
    pass = ENV.fetch('DB_PASS', 'postgres')
    name = ENV.fetch('DB_NAME', 'smallpict')
    ssl  = ENV.fetch('DB_SSLMODE', 'disable')

    ENV.fetch('DATABASE_URL', "postgres://#{user}:#{pass}@#{host}:#{port}/#{name}?sslmode=#{ssl}")
  end

  def self.smallpict_mode
    ENV.fetch('SMALLPICT_MODE', 'locale').downcase
  end

  def self.smallpict_base_url
    ENV.fetch('SMALLPICT_BASE_URL', 'https://api.smallpict.app')
  end

  def self.smallpict_api_key
    ENV.fetch('SMALLPICT_API_KEY', '')
  end

  def self.smallpict_secret_key
    ENV.fetch('SMALLPICT_SECRET_KEY', '')
  end

  def self.smallpict_format
    ENV.fetch('SMALLPICT_FORMAT', 'auto')
  end

  def self.smallpict_quality
    ENV.fetch('SMALLPICT_QUALITY', 80).to_i
  end

  def self.upload_path
    ENV.fetch('UPLOAD_PATH', './uploads')
  end
end
