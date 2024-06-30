#!/usr/bin/ruby
require 'io/console'

def stage_to_string(stage, lambda_y, lambda_x)
    buffer = ""
    stage.each_with_index do |row, y|
        row.each_with_index do |c, x|
            if y == lambda_y && x == lambda_x
                buffer += "L "
            else
                buffer += "#{c} "
            end
        end
        buffer += "\n"
    end
    buffer
end

def can_move_to(stage, y, x)
    y >= 0 && y < stage.size && x >= 0 && x < stage[y].size && stage[y][x] != '#'
end

State = Struct::new('State', :stage, :lambda_y, :lambda_x, :history)
def main
    if ARGV.size == 0
        $stderr.puts "Usage: handman STAGE"
    end

    stage_txt = File.read(ARGV[0])

    stage = []
    start = nil
    stage_txt.lines.each_with_index do |line, y|
        row = []
        line.chars.each_with_index do |c, x|
            if c == '#'
                row.push('#')
            elsif c == '.'
                row.push('.')
            elsif c == 'L'
                row.push(' ')
                start = [y, x]
            end
        end
        stage.push(row)
    end

    states = []
    states.push(State.new(stage, start[0], start[1], ""))
    loop do
        state = states.last
        stage = state.stage
        lambda_y = state.lambda_y
        lambda_x = state.lambda_x
        history = state.history

        puts(
            "[#{history.size}] #{history}\n" +
            "#{stage_to_string(stage, lambda_y, lambda_x)}" +
            "[hjkl][u][q]: "
        )

        dy = 0
        dx = 0
        op = nil
        input = $stdin.getch
        case input
        when 'h' then
            dx = -1
            op = 'L'
        when 'j' then
            dy = 1
            op = 'D'
        when 'k' then
            dy = -1
            op = 'U'
        when 'l' then
            dx = 1
            op = 'R'
        when 'u' then
            states.pop
            next
        when 'q' then
            break
        else
            puts "invalid input"
            next
        end
        unless can_move_to(stage, lambda_y+dy, lambda_x+dx)
            puts "cannot move to y=#{lambda_y+dy} x=#{lambda_x+dx}"
            next
        end
        new_stage = Marshal.load(Marshal.dump(stage))
        new_stage[lambda_y][lambda_x] = ' '
        new_state = State.new(
            new_stage,
            lambda_y + dy,
            lambda_x + dx,
            history + op
        )
        states.push(new_state)
    end

    puts states.last.history
end

main
