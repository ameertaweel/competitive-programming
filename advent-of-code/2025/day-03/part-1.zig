const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var joltage_max: u64 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const trimmed = std.mem.trim(u8, line, "\n");

        var max: i64 = -1;
        var max_after_max: i64 = -1;
        for (0..trimmed.len - 1) |i| {
            const digit: i32 = trimmed[i] - '0';

            if (digit > max) {
                max = digit;
                max_after_max = -1;
                continue;
            }

            if (digit > max_after_max) {
                max_after_max = digit;
            }
        }

        const digitFinal: i32 = trimmed[trimmed.len - 1] - '0';
        if (digitFinal > max_after_max) {
            max_after_max = digitFinal;
        }

        // DEBUGGING
        print("MAX: {}, MAX AFTER MAX: {}\n", .{ max, max_after_max });

        const joltage_out: u64 = @intCast(max * 10 + max_after_max);

        joltage_max += joltage_out;
    }

    print("{d}\n", .{joltage_max});
}
