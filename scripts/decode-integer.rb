input = $stdin.read.strip

if input[0] != "I"
  raise RuntimeError("not a integer")
end
input = input[1..]

ret = 0
for c in input.chars
  ret = ret * 94 + c.ord - 33
end
puts ret

