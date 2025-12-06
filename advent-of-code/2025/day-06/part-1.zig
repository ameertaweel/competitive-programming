const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_LINES = 10;
const MAX_PROBLEMS = 1000;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [MAX_LINES * MAX_PROBLEMS]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var data: [MAX_LINES][MAX_PROBLEMS]u64 = [_][MAX_PROBLEMS]u64{[_]u64{0} ** MAX_PROBLEMS} ** MAX_LINES;

    var problems_count: usize = 0;
    var lines_count: usize = 0;

    var total: u64 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        problems_count = 0;

        var it = std.mem.splitScalar(u8, line, ' ');
        while (it.next()) |word| {
            if (word.len == 0) continue;
            // DEBUGGING
            // print("WORD {s}\n", .{word});
            if (word[0] == '+') {
                var ans: u64 = 0;
                for (0..lines_count) |j| {
                    // DEBUGGING
                    // print("OP {d}\n", .{data[j][problems_count]});
                    ans += data[j][problems_count];
                }
                // DEBUGGING
                // print("ADD: {d}\n", .{ans});
                total += ans;
                problems_count += 1;
                continue;
            }
            if (word[0] == '*') {
                var ans: u64 = 1;
                for (0..lines_count) |j| {
                    // DEBUGGING
                    // print("OP {d}\n", .{data[j][problems_count]});
                    ans *= data[j][problems_count];
                }
                // DEBUGGING
                // print("MUL: {d}\n", .{ans});
                total += ans;
                problems_count += 1;
                continue;
            }
            data[lines_count][problems_count] = try std.fmt.parseInt(u64, word, 10);
            problems_count += 1;
        }

        lines_count += 1;
    }

    print("{d}\n", .{total});
}
