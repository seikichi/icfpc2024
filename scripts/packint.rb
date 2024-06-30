#!/usr/bin/ruby
input = $stdin.read.strip

ret = 0
for c in input.chars do
    i = "LRDU".index(c)
    ret = ret * 4 + i
end
puts ret
