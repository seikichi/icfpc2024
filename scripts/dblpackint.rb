#!/usr/bin/ruby

def encode_int(n)
    if n == 0
        return "I!"
    end
    ret = ""
    while n > 0
        ret += (n % 94 + 33).chr
        n /= 94
    end
    "I" + ret.reverse
end

def encode_string(input)
    table = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    rtable = {}

    table.chars.each_with_index do |c, i|
        rtable[c] = i
    end

    ret = "S"
    for c in input.chars
        ret += (rtable[c].ord + 33).chr
    end
    ret
end

def decoder(n, problem)
    code = encode_int(n)
    prefix = encode_string("solve #{problem} ")
    %|B$ B$ L! B$ v! v! L" L# ? B= v# I" #{prefix} B. B$ B$ v" v" B/ v# I% B$ L$ B. v$ v$ BT I" BD B% v# I% SFL>O #{code}|
end

problem = ARGV[0]
input = $stdin.read.strip

ret = 1
for c in input.chars do
    i = "LRDU".index(c)
    ret = ret * 4 + i
end
puts decoder(ret, problem)
