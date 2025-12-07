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

    var beams: [MAX_COLS]usize = [_]usize{0} ** MAX_COLS;
    var beams_next: [MAX_COLS]usize = [_]usize{0} ** MAX_COLS;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        cols = line.len;
        if (cols >= MAX_COLS) @panic("Too many columns.");

        for (0..cols) |col| {
            if (line[col] == 'S') {
                beams_next[col] = 1;
                break;
            }

            if (line[col] != '^') {
                beams_next[col] += beams[col];
            } else {
                if (col > 0) beams_next[col - 1] += beams[col];
                if (col < cols - 1) beams_next[col + 1] += beams[col];
            }
        }

        for (0..cols) |col| {
            beams[col] = 0;
            if (line[col] == '^') beams_next[col] = 0;
        }

        const temp = beams;
        beams = beams_next;
        beams_next = temp;
    }

    var timelines: usize = 0;
    for (0..cols) |col| {
        timelines += beams[col];
    }

    print("{d}\n", .{timelines});
}
