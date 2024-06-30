#!/usr/bin/ruby
input = $stdin.read.strip

ret = 1
for c in input.chars do
    i = "LRDU".index(c)
    ret = ret * 4 + i
end
puts ret
