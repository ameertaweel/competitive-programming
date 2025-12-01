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

        const n = sig * mag;

        dial += n;
        while (dial > 99) {
            dial -= 100;
        }
        while (dial < 0) {
            dial += 100;
        }

        // DEBUGGING
        // print("{d} => {d}\n", .{n, dial});

        if (dial == 0) {
            zeroes += 1;
        }
    }

    print("{d}\n", .{zeroes});
}
