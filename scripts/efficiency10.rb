$field = []
$fixed = []
9.times do
    $field.push([0]*9)
    $fixed.push([false]*9)
end

# ((v17 == 6)) && (v18 == 8)) && (v25 == 7)) && (v26 == 3)) && (v29 == 9)) && (v31 == 3))
# && (v33 == 9)) && (v38 == 4)) && (v39 == 5)) && (v41 == 4)) && (v42 == 9)) && (v51 == 8))
# && (v53 == 3)) && (v55 == 5)) && (v57 == 9)) && (v59 == 2)) && (v68 == 3)) && (v69 == 6))
# && (v71 == 9)) && (v72 == 6)) && (v77 == 3)) && (v79 == 8)) && (v81 == 7)) && (v84 == 6))
# && (v85 == 8)) && (v92 == 2)) && (v93 == 8))
fill = [
    [1, 7, 6],
    [1, 8, 8],
    [2, 5, 7],
    [2, 6, 3],
    [2, 9, 9],
    [3, 1, 3],

    [3, 3, 9],
    [3, 8, 4],
    [3, 9, 5],
    [4, 1, 4],
    [4, 2, 9],
    [5, 1, 8],

    [5, 3, 3],
    [5, 5, 5],
    [5, 7, 9],
    [5, 9, 2],
    [6, 8, 3],
    [6, 9, 6],

    [7, 1, 9],
    [7, 2, 6],
    [7, 7, 3],
    [7, 9, 8],
    [8, 1, 7],
    [8, 4, 6],

    [8, 5, 8],
    [9, 2, 2],
    [9, 3, 8],
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
