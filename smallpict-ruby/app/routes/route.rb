module Routes
  module Route
    def self.registered(app)
      app.get '/' do
        content_type :json
        {
          status: 'ok',
          service: AppConfig.app_name,
          version: '1.0.0'
        }.to_json
      end

      app.get '/health' do
        content_type :json
        db_ok = DatabaseHealth.healthy?
        status db_ok ? 200 : 503

        {
          status: db_ok ? 'ok' : 'unhealthy',
          database: db_ok ? 'connected' : 'disconnected',
          timestamp: Time.now.utc.iso8601
        }.to_json
      end

      upload_handler = lambda do
        content_type :json

        req_type = request.content_type.to_s.downcase

        if req_type.include?('application/json')
          body_content = request.body.read
          begin
            json_body = JSON.parse(body_content)
          rescue StandardError
            halt 400, { status: 'error', message: 'Invalid JSON body' }.to_json
          end

          base64_str = json_body['image'] || json_body['base64'] || json_body['data']
          filename = json_body['name'] || json_body['filename']

          if base64_str.nil? || base64_str.empty?
            halt 400, { status: 'error', message: "Field 'image' or 'base64' is required" }.to_json
          end

          begin
            record = SmallPictHelper.upload(base64_str: base64_str, original_filename: filename)
            {
              status: 'success',
              message: 'Image uploaded successfully',
              data: record.to_h
            }.to_json
          rescue StandardError => e
            halt 500, { status: 'error', message: e.message }.to_json
          end

        else
          file_param = params[:file] || params[:image]
          filename = params[:name] || params[:filename]
          base64_str = params[:base64] || params[:image] if params[:image].is_a?(String)

          if file_param.nil? && (base64_str.nil? || base64_str.empty?)
            halt 400, { status: 'error', message: 'No file or image payload found' }.to_json
          end

          begin
            record = SmallPictHelper.upload(
              file_hash: file_param.is_a?(Hash) ? file_param : nil,
              base64_str: base64_str.is_a?(String) ? base64_str : nil,
              original_filename: filename
            )
            {
              status: 'success',
              message: 'Image uploaded successfully',
              data: record.to_h
            }.to_json
          rescue StandardError => e
            halt 500, { status: 'error', message: e.message }.to_json
          end
        end
      end

      app.post '/upload', &upload_handler
      app.not_found do
        content_type :json
        { status: 'error', message: "Route not found: #{request.request_method} #{request.path_info}" }.to_json
      end
    end
  end
end
