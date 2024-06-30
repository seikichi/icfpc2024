#!/usr/bin/ruby

def run_length(input)
    if input.size == 0
        return []
    end
    prev = input[0]
    run = 1
    ret = []
    for c in input[1..].chars do
        if c == prev
            run += 1
        else
            ret.push([prev, run])
            prev = c
            run = 1
        end
    end
    ret.push([prev, run])
    ret
end

def split_too_large_runs(runs, n_length_bits)
    max_run = 2**n_length_bits - 1
    ret = []
    for c, run in runs
        while run > max_run
            ret.push([c, max_run])
            run -= max_run
        end
        ret.push([c, run])
    end
    ret
end

def runs_to_int(runs, n_length_bits)
    ret = 1
    for c, run in runs
        if run >= 2**n_length_bits
            raise "run is too large: #{c} #{run}"
        end
        ret = (ret << n_length_bits) + run
        d = "LRDU".index(c)
        ret = (ret << 2) + d
    end
    ret
end

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
    code = encode_int(n)
    %|B$ L! B$ B$ v! L$ L% ? B= v% I! S B. B$ v$ B/ v% I#e B$ B$ L& B$ v! L$ L' ? B= v' I! S B. B$ v$ B- v' I" v& BT I" BD B% v% I% SFL>O B% B/ v% I% Ia #{code} L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v#|
end

N_LENGTH_BITS = 6
input = $stdin.read.strip
runs = run_length(input)
runs = split_too_large_runs(runs, N_LENGTH_BITS)
encoded = runs_to_int(runs, N_LENGTH_BITS)
puts decoder(encoded)
