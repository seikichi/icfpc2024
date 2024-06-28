#!/usr/bin/ruby

require 'net/http'
require 'uri'
require 'json'

if ARGV.size == 0
    input = $stdin.read.strip
elsif ARGV.size == 1
    input = ARGV[0]
else
    $stderr.puts "Usage: communicate.rb [INPUT]"
end

# リクエスト先のURL
url = URI.parse('https://boundvariable.space/communicate')

# リクエストの準備
http = Net::HTTP.new(url.host, url.port)
http.use_ssl = true
request = Net::HTTP::Post.new(url.request_uri)

request['Authorization'] = 'Bearer SET_YOUR_TOKEN'

# リクエストボディの設定
request.body = input

# リクエストの送信とレスポンスの取得
response = http.request(request)

# レスポンスの表示
puts "#{response.body}"
