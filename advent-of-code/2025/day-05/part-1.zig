const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_RANGES = 500;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var ranges: [MAX_RANGES][2]u64 = [_][2]u64{[_]u64{0} ** 2} ** MAX_RANGES;

    var ranges_len: usize = 0;
    var ranges_done: bool = false;

    var valid: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        if (line.len == 0) {
            ranges_done = true;
            continue;
        }

        if (!ranges_done) {
            var it = std.mem.splitSequence(u8, line, "-");
            const beg = try std.fmt.parseInt(u64, it.next().?, 10);
            const end = try std.fmt.parseInt(u64, it.next().?, 10);
            ranges[ranges_len][0] = beg;
            ranges[ranges_len][1] = end;
            ranges_len += 1;
            continue;
        }

        const id = try std.fmt.parseInt(u64, line, 10);

        for (0..ranges_len) |idx| {
            if (ranges[idx][0] <= id and id <= ranges[idx][1]) {
                valid += 1;
                break;
            }
        }
    }

    print("{d}\n", .{valid});
}
