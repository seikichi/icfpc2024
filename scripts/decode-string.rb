#!/usr/bin/ruby

input = $stdin.read.strip
if input[0] != "S" then
    raise RuntimeError("not a string")
end

TABLE = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"

ret = ""
for c in input[1..].chars
    ret += TABLE[c.ord - 33]
end
puts ret
