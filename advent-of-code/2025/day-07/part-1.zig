const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_COLS = 500;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var cols: usize = 0;

    var beams: [MAX_COLS]bool = [_]bool{false} ** MAX_COLS;
    var beams_next: [MAX_COLS]bool = [_]bool{false} ** MAX_COLS;

    var splits: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        cols = line.len;
        if (cols >= MAX_COLS) @panic("Too many columns.");

        for (0..cols) |col| {
            if (line[col] == 'S') {
                beams_next[col] = true;
                break;
            }

            if (line[col] != '^') {
                if (beams[col]) beams_next[col] = true;
            } else {
                if (beams[col]) {
                    splits += 1;
                    if (col > 0) beams_next[col - 1] = true;
                    if (col < cols - 1) beams_next[col + 1] = true;
                }
            }

            beams[col] = false;
        }

        for (0..cols) |col| {
            if (line[col] == '^') beams_next[col] = false;
        }

        const temp = beams;
        beams = beams_next;
        beams_next = temp;
    }

    print("{d}\n", .{splits});
}
