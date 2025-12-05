const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_ROWS = 500;
const MAX_COLS = 500;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var map: [MAX_ROWS][MAX_COLS]u8 = [_][MAX_COLS]u8{[_]u8{0} ** MAX_COLS} ** MAX_ROWS;

    var rows: usize = 0;
    var cols: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        rows += 1;
        cols = line.len;

        if (rows >= MAX_ROWS) @panic("Too many rows.");
        if (cols >= MAX_COLS) @panic("Too many columns.");

        for (0..cols) |col| {
            map[rows][col + 1] = if (line[col] == '@') 1 else 0;
        }
    }

    var ans: u32 = 0;

    for (0..rows) |row| {
        for (0..cols) |col| {
            if (map[row + 1][col + 1] == 0) continue;

            const neighbors: u8 = map[row + 0][col + 0] + map[row + 0][col + 1] + map[row + 0][col + 2] + map[row + 1][col + 2] + map[row + 2][col + 2] + map[row + 2][col + 1] + map[row + 2][col + 0] + map[row + 1][col + 0];

            if (neighbors < 4) {
                ans += 1;
            }
        }
    }

    print("{d}\n", .{ans});
}
