#!/usr/bin/ruby

input = $stdin.read.strip

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
