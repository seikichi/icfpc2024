$field = []
9.times do
    $field.push([0]*9)
end

def check(y, x)
    n = $field[y][x]
    for xx in 0...9 do
        next if xx == x
        return false if $field[y][xx] == n
    end
    for yy in 0...9 do
        next if yy == y
        return false if $field[yy][x] == n
    end
    ox = (x / 3) * 3
    oy = (y / 3) * 3
    for i in 0...9 do
        xx = ox + i % 3
        yy = oy + i / 3
        next if yy == y && xx == x
        return false if $field[yy][xx] == n
    end
    true
end

def print_field
    for y in 0...9 do
        p $field[y]
    end
end

def encode_field
    ret = 0
    for y in 0...9 do
        for x in 0...9 do
            ret = ret * 9 + $field[y][x] - 1
        end
    end
    ret
end

def solve(y, x)
    if y == 9
        print_field
        p encode_field
        exit 0
    end
    if x == 9
        solve(y+1, 0)
    end
    for i in 1..9 do
        $field[y][x] = i
        if check(y, x)
            solve(y, x+1)
        end
    end
    $field[y][x] = 0
end

solve(0, 0)
