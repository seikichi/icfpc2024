input = $stdin.read.strip

field = []
depth_map = []
visit = []
start = nil
$remains = 0
input.lines(chomp: true).each_with_index do |line, y|
    row = []
    line.chars.each_with_index do |c, x|
        if c == 'L'
            start = [y, x]
            row.push('.')
            $remains += 1
        elsif c == '.'
            row.push('.')
            $remains += 1
        else
            row.push('#')
        end
    end
    field.push(row)
    depth_map.push([-1] * row.size)
    visit.push([false] * row.size)
end

$dx = [-1, 0, 1, 0]
$dy = [0, 1, 0, -1]
$symbol = "LDRU"
$rsymbol = "RULD"
def dfs(field, depth_map, visit, y, x, depth)
    visit[y][x] = true
    max_depth = depth
    for dir in 0..3 do
        yy = y + $dy[dir]
        xx = x + $dx[dir]
        if 0 <= yy && yy < field.size && 0 <= xx && xx < field[yy].size && field[yy][xx] == '.' && !visit[yy][xx]
            d = dfs(field, depth_map, visit, yy, xx, depth+1)
            max_depth = [max_depth, d].max
        end
    end
    depth_map[y][x] = max_depth
end
dfs(field, depth_map, visit, start[0], start[1], 0)

# clear visit
for i in 0...visit.size do
    for j in 0...visit[i].size do
        visit[i][j] = false
    end
end

$path = ""
def dfs2(field, depth_map, visit, y, x)
    visit[y][x] = true
    $remains -= 1
    if $remains == 0
        puts $path
        exit 0
    end
    # max_depth が浅い順に探索する
    dirs = [0, 1, 2, 3].sort_by {|dir|
        yy = y + $dy[dir]
        xx = x + $dx[dir]
        if 0 <= yy && yy < field.size && 0 <= xx && xx < field[yy].size && field[yy][xx] == '.' && !visit[yy][xx]
            depth_map[yy][xx]
        else
            -1 # visit 不可能な方向はどうでもいい
        end
    }
    for dir in dirs do
        yy = y + $dy[dir]
        xx = x + $dx[dir]
        if 0 <= yy && yy < field.size && 0 <= xx && xx < field[yy].size && field[yy][xx] == '.' && !visit[yy][xx]
            $path += $symbol[dir]
            dfs2(field, depth_map, visit, yy, xx)
            $path += $rsymbol[dir]
        end
    end
end
dfs2(field, depth_map, visit, start[0], start[1])

