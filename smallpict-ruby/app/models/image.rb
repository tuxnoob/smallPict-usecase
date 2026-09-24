require_relative '../database'

class Image < Sequel::Model(:images)
  unrestrict_primary_key
  plugin :timestamps, update_on_create: true

  def to_h
    {
      id: id,
      filename: filename,
      url: url,
      size: size,
      size_origin: size_origin,
      mime_type: mime_type,
      created_at: created_at ? created_at.iso8601 : nil
    }
  end
end
