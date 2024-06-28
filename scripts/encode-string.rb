#!/usr/bin/ruby

if ARGV.size == 0
    input = $stdin.read.strip
elsif ARGV.size == 1
    input = ARGV[0]
else
    $stderr.puts "Usage: encode-string.rb [INPUT]"
end

table = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
rtable = {}

table.chars.each_with_index do |c, i|
    rtable[c] = i
end

ret = "S"
for c in input.chars
    ret += (rtable[c].ord + 33).chr
end
puts ret
