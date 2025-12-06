const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_LINES = 10;
const MAX_LINE_LEN = 4000;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [MAX_LINES * MAX_LINE_LEN]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var lines: [MAX_LINES][MAX_LINE_LEN]u8 = [_][MAX_LINE_LEN]u8{[_]u8{0} ** MAX_LINE_LEN} ** MAX_LINES;
    var lines_count: usize = 0;
    var line_len: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        std.mem.copyForwards(u8, &lines[lines_count], line);
        lines_count += 1;
        line_len = line.len;
    }

    var total: u64 = 0;
    var acc: u64 = 0;
    var op: u8 = ' ';
    for (0..line_len) |col| {
        var empty: usize = 0;
        for (0..lines_count) |row| {
            if (lines[row][col] == ' ') empty += 1;
        }
        if (empty == lines_count) {
            total += acc;
            continue;
        }

        const op_next = lines[lines_count - 1][col];
        if (op_next == '+') {
            op = '+';
            acc = 0;
        }
        if (op_next == '*') {
            op = '*';
            acc = 1;
        }

        var num: u64 = 0;
        for (0..lines_count - 1) |row| {
            if (lines[row][col] == ' ') continue;
            num *= 10;
            num += lines[row][col] - '0';
        }

        acc = if (op == '+') acc + num else acc * num;
    }

    total += acc;

    print("{d}\n", .{total});
}
