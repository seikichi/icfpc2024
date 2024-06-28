#!/usr/bin/ruby

if ARGV.size == 0
    input = $stdin.read.strip
elsif ARGV.size == 1
    input = ARGV[0]
else
    $stderr.puts "Usage: decode-string.rb [INPUT]"
end

if input[0] != "S" then
    raise RuntimeError("not a string")
end

TABLE = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"

ret = ""
for c in input[1..].chars
    ret += TABLE[c.ord - 33]
end
puts ret
