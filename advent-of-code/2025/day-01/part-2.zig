const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [10]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var dial: i32 = 50;
    var zeroes: i32 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const sig: i32 = if (line[0] == 'L') -1 else 1;
        const mag: i32 = try std.fmt.parseInt(i32, line[1..line.len], 10);

        zeroes += @divTrunc(mag, 100);

        if (@mod(mag, 100) == 0) continue;

        const n = sig * @mod(mag, 100);

        const dial_next = dial + n;

        if (dial_next > 99) {
            zeroes += 1;
            dial = dial_next - 100;
        } else if (dial_next == 0) {
            zeroes += 1;
            dial = dial_next;
        } else if (dial_next < 0) {
            if (dial != 0) {
                zeroes += 1;
            }
            dial = dial_next + 100;
        } else {
            dial = dial_next;
        }

        // DEBUGGING
        // print("{d} => {d}\n", .{zeroes, dial});
    }

    print("{d}\n", .{zeroes});
}
