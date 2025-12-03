const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const BATTERIES_PER_BANK = 12;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var joltage_max: u64 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const trimmed = std.mem.trim(u8, line, "\n");

        var joltages = [_]i64{-1} ** BATTERIES_PER_BANK;

        for (0..trimmed.len) |i| {
            const digit: i32 = trimmed[i] - '0';

            const rem = trimmed.len - i;
            for (0..BATTERIES_PER_BANK) |j| {
                if (rem + j < BATTERIES_PER_BANK) continue;

                if (digit > joltages[j]) {
                    joltages[j] = digit;
                    for ((j + 1)..BATTERIES_PER_BANK) |k| {
                        joltages[k] = -1;
                    }
                    break;
                }
            }
        }

        // DEBUGGING
        for (joltages) |v| {
            print("{d}, ", .{v});
        }
        print("\n", .{});

        var joltage_out: u64 = 0;
        for (0..BATTERIES_PER_BANK) |i| {
            joltage_out = joltage_out * 10 + @as(u64, @intCast(joltages[i]));
        }

        // DEBUGGING
        print("{d}\n", .{joltage_out});

        joltage_max += joltage_out;
    }

    print("{d}\n", .{joltage_max});
}
