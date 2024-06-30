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

def decoder(n)
    body = 'BD I" B$ B$ L! B$ L" B$ v! B$ v" v" L" B$ v! B$ v" v" L# L$ ? B= v$ I! S B. B$ v# B/ v$ I% BT I" BD B% v$ I% SFL>O '
    body + encode_int(n)
end

input = $stdin.read.strip

ret = 1
for c in input.chars do
    i = "LRDU".index(c)
    ret = ret * 4 + i
end
puts decoder(ret)
