const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_RANGES = 500;

pub fn ranges_intersect(beg1: u64, end1: u64, beg2: u64, end2: u64) bool {
    if (beg2 <= beg1 and beg1 <= end2) return true;
    if (beg2 <= end1 and end1 <= end2) return true;
    if (beg1 <= beg2 and beg2 <= end1) return true;
    if (beg1 <= end2 and end2 <= end1) return true;
    return false;
}

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var ranges: [MAX_RANGES][2]u64 = [_][2]u64{[_]u64{0} ** 2} ** MAX_RANGES;

    var ranges_len: usize = 0;
    var ranges_done: bool = false;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        if (line.len == 0) {
            ranges_done = true;
            break;
        }

        var it = std.mem.splitSequence(u8, line, "-");
        var beg1 = try std.fmt.parseInt(u64, it.next().?, 10);
        var end1 = try std.fmt.parseInt(u64, it.next().?, 10);

        for (0..ranges_len) |idx| {
            const beg2 = ranges[idx][0];
            const end2 = ranges[idx][1];

            if (beg2 > end2) continue;
            if (!ranges_intersect(beg1, end1, beg2, end2)) continue;

            beg1 = @min(beg1, beg2);
            end1 = @max(end1, end2);

            ranges[idx][0] = 1;
            ranges[idx][1] = 0;
        }

        ranges[ranges_len][0] = beg1;
        ranges[ranges_len][1] = end1;

        // DEBUGGING
        // print("{d} {d}\n", .{ beg1, end1 });

        ranges_len += 1;
    }

    var valid: u64 = 0;

    for (0..ranges_len) |idx| {
        const beg = ranges[idx][0];
        const end = ranges[idx][1];
        if (beg > end) continue;
        valid += end - beg + 1;
    }

    print("{d}\n", .{valid});
}
