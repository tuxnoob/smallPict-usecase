require 'sinatra/base'
require 'json'
require_relative 'app/config'
require_relative 'app/database'
require_relative 'app/helpers/smallpict_helper'
require_relative 'app/routes/route'

class App < Sinatra::Base
  set :port, AppConfig.port
  set :bind, AppConfig.host

  FileUtils.mkdir_p(AppConfig.upload_path)
  set :public_folder, File.dirname(AppConfig.upload_path)

  register Routes::Route

  run! if app_file == $PROGRAM_NAME
end
