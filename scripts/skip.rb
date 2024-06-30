#!/usr/bin/ruby
input = $stdin.read.strip
ret = ""
input.chars.each_with_index do |c, i|
    if i % 2 == 0
        ret += c
    end
end
puts ret
