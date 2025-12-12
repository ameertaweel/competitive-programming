const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_POINTS = 1000;

const PointGrid = struct {
    row: usize,
    col: usize,

    pub fn square_area(self: *const @This(), other: *const @This()) usize {
        const w = (if (self.row > other.row) (self.row - other.row) else (other.row - self.row)) + 1;
        const h = (if (self.col > other.col) (self.col - other.col) else (other.col - self.col)) + 1;
        // DEBUGGING
        // print("({d}, {d}) * ({d}, {d}) = ({d}, {d})\n", .{ self.row, self.col, other.row, other.col, w, h });
        return w * h;
    }
};

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var points: [MAX_POINTS]PointGrid = undefined;
    var points_len: usize = 0;

    var max_area: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var it = std.mem.splitScalar(u8, line, ',');
        const row = try std.fmt.parseInt(usize, it.next().?, 10);
        const col = try std.fmt.parseInt(usize, it.next().?, 10);

        const p = PointGrid{ .row = row, .col = col };

        for (0..points_len) |i| {
            const area = points[i].square_area(&p);
            if (area > max_area) max_area = area;
        }

        points[points_len] = p;
        points_len += 1;
    }

    print("{d}\n", .{max_area});
}
