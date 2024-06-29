$field = []
$fixed = []
9.times do
    $field.push([0]*9)
    $fixed.push([false]*9)
end

# (v12 == 6)) && (v13 == 4)) && (v17 == 7)) && (v25 == 2)) && (v28 == 3)) && (v29 == 6)) &&
# (v33 == 1)) && (v41 == 2)) && (v42 == 3)) && (v45 == 8)) && (v54 == 7)) && (v57 == 1)) &&
# (v59 == 4)) && (v71 == 9)) && (v81 == 8)) && (v88 == 2)) && (v94 == 4))
fill = [
    [1, 2, 6],
    [1, 3, 4],
    [1, 7, 7],
    [2, 5, 2],
    [2, 8, 3],
    [2, 9, 6],

    [3, 3, 1],
    [4, 1, 2],
    [4, 2, 3],
    [4, 5, 8],
    [5, 4, 7],
    [5, 7, 1],

    [5, 9, 4],
    [7, 1, 9],
    [8, 1, 8],
    [8, 8, 2],
    [9, 4, 4],
]
for y, x, v in fill do
    $field[y-1][x-1] = v
    $fixed[y-1][x-1] = true
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
        return
    end
    if $fixed[y][x]
        solve(y, x+1)
        return
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
