const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_LINE_LEN = 1000;
const MAX_SHAPES = 1000;
const SHAPE_WIDTH = 3;
const SHAPE_HEIGHT = 3;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [MAX_LINE_LEN]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var shapes: [MAX_SHAPES]u64 = undefined;
    var shapes_len: usize = 0;

    var shapes_done = true;

    var valid: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        if (line.len == 0) {
            if (!shapes_done) shapes_len += 1;
            shapes_done = true;
            continue;
        }
        if (line[line.len - 1] == ':') {
            shapes_done = false;
            shapes[shapes_len] = 0;
            continue;
        }

        if (!shapes_done) {
            for (0..line.len) |i| {
                if (line[i] == '#') shapes[shapes_len] += 1;
            }
            continue;
        }

        // Shapes Done
        var it = std.mem.splitSequence(u8, line, ": ");
        const dims = it.next().?;
        const qtys = it.next().?;

        var it_dims = std.mem.splitScalar(u8, dims, 'x');
        const w = try std.fmt.parseInt(usize, it_dims.next().?, 10);
        const h = try std.fmt.parseInt(usize, it_dims.next().?, 10);

        var needed_shapes: usize = 0;
        var needed_blocks: usize = 0;

        var it_qtys = std.mem.splitScalar(u8, qtys, ' ');
        for (0..shapes_len) |i| {
            const q = try std.fmt.parseInt(usize, it_qtys.next().?, 10);
            needed_shapes += q;
            needed_blocks += q * shapes[i];
        }

        if (needed_blocks > w * h) {
            // print("Not enough blocks!\n", .{});
            continue;
        }
        if (needed_shapes <= (w / SHAPE_WIDTH) * (h / SHAPE_HEIGHT)) {
            // print("More than enough space!\n", .{});
            valid += 1;
            continue;
        }
        print("Unknown!\n", .{});
    }

    print("{d}\n", .{valid});
}
