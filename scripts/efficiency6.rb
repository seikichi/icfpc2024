# n > 30 なる n について、is_prime(fib(n)) となる最小の n
require 'prime'

$memo = {}

def fib(n)
    return $memo[n] if $memo[n] != nil
    if n < 2
        $memo[n] = 1
        return 1
    end
    $memo[n] = fib(n-1) + fib(n-2)
    $memo[n]
end

for n in 31.. do
    puts "n=#{n}"
    if fib(n).prime?
        puts n
        return
    end
end