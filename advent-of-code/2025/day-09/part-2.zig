const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_POINTS = 1000;

const GridPoint = struct {
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

const GridLineH = struct {
    row: usize,
    col_l: usize,
    col_r: usize,
};

const GridLineV = struct {
    col: usize,
    row_t: usize,
    row_b: usize,
};

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var points: [MAX_POINTS]GridPoint = undefined;
    var points_len: usize = 0;

    var lines_h: [(MAX_POINTS + 1) / 2]GridLineH = undefined;
    var lines_h_len: usize = 0;

    var lines_v: [(MAX_POINTS + 1) / 2]GridLineV = undefined;
    var lines_v_len: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var it = std.mem.splitScalar(u8, line, ',');
        const row = try std.fmt.parseInt(usize, it.next().?, 10);
        const col = try std.fmt.parseInt(usize, it.next().?, 10);

        const p = GridPoint{ .row = row, .col = col };

        for (0..points_len) |i| {
            const q = points[i];
            if (p.row == q.row) {
                lines_h[lines_h_len] = GridLineH{
                    .row = p.row,
                    .col_l = @min(p.col, q.col),
                    .col_r = @max(p.col, q.col),
                };
                lines_h_len += 1;
            }
            if (p.col == q.col) {
                lines_v[lines_v_len] = GridLineV{
                    .col = p.col,
                    .row_t = @min(p.row, q.row),
                    .row_b = @max(p.row, q.row),
                };
                lines_v_len += 1;
            }
        }

        points[points_len] = p;
        points_len += 1;
    }

    var max_area: usize = 0;
    for (0..points_len) |a| {
        const pa = points[a];
        mid: for (0..a) |b| {
            const pb = points[b];
            const area = pa.square_area(&pb);
            if (area <= max_area) continue;

            const row_top = @min(pa.row, pb.row);
            const row_bot = @max(pa.row, pb.row);
            const col_lft = @min(pa.col, pb.col);
            const col_rgt = @max(pa.col, pb.col);

            for (lines_h[0..lines_h_len]) |l| {
                if (row_top < l.row and l.row < row_bot and col_lft < l.col_r and l.col_l < col_rgt) continue :mid;
            }
            for (lines_v[0..lines_v_len]) |l| {
                if (col_lft < l.col and l.col < col_rgt and row_top < l.row_b and l.row_t < row_bot) continue :mid;
            }

            // DEBUGGING
            // print("({d}, {d}) and ({d}, {d}) => {d}\n", .{ pa.row, pa.col, pb.row, pb.col, area });

            max_area = area;
        }
    }
    print("{d}\n", .{max_area});
}
