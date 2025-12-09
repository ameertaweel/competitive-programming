const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_POINTS = 1000;

pub fn UnionFind(comptime MAX_SIZE: usize) type {
    return struct {
        parent: [MAX_SIZE]usize,
        size: [MAX_SIZE]usize,

        pub fn init() UnionFind(MAX_SIZE) {
            return .{
                .parent = undefined,
                .size = undefined,
            };
        }

        pub fn insert(self: *@This(), v: usize) void {
            self.parent[v] = v;
            self.size[v] = 1;
        }

        pub fn find(self: *@This(), v: usize) usize {
            if (self.parent[v] == v) return v;
            const p = self.find(self.parent[v]);
            self.parent[v] = p;
            return p;
        }

        pub fn merge(self: *@This(), a: usize, b: usize) usize {
            const pa = self.find(a);
            const pb = self.find(b);

            if (pa != pb) {
                if (self.size[pa] < self.size[pb]) {
                    self.parent[pa] = pb;
                    self.size[pb] += self.size[pa];
                    return self.size[pb];
                } else {
                    self.parent[pb] = pa;
                    self.size[pa] += self.size[pb];
                    return self.size[pa];
                }
            }

            return @max(self.size[pa], self.size[pb]);
        }
    };
}

const Distance = struct {
    a: usize,
    b: usize,
    dist: f64,

    pub fn cmp(_: void, a: @This(), b: @This()) bool {
        return a.dist < b.dist;
    }
};

const Point3D = struct {
    x: i64,
    y: i64,
    z: i64,
    idx: usize,

    pub fn dist(self: *const @This(), other: *const @This()) Distance {
        const a = (self.x - other.x) * (self.x - other.x);
        const b = (self.y - other.y) * (self.y - other.y);
        const c = (self.z - other.z) * (self.z - other.z);
        const d = std.math.sqrt(@as(f64, @floatFromInt(a + b + c)));

        return .{ .a = self.idx, .b = other.idx, .dist = d };
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var points: [MAX_POINTS]Point3D = undefined;
    var points_len: usize = 0;

    var distances = try allocator.alloc(Distance, (MAX_POINTS * (MAX_POINTS - 1)) / 2);
    defer allocator.free(distances);
    var distances_len: usize = 0;

    var uf = UnionFind(MAX_POINTS).init();

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var it = std.mem.splitScalar(u8, line, ',');
        const x = try std.fmt.parseInt(i64, it.next().?, 10);
        const y = try std.fmt.parseInt(i64, it.next().?, 10);
        const z = try std.fmt.parseInt(i64, it.next().?, 10);
        const p = Point3D{ .x = x, .y = y, .z = z, .idx = points_len };

        for (0..points_len) |i| {
            const dist = points[i].dist(&p);
            distances[distances_len] = dist;
            distances_len += 1;
        }

        points[points_len] = p;
        uf.insert(points_len);
        points_len += 1;
    }

    std.mem.sort(Distance, distances[0..distances_len], {}, Distance.cmp);
    for (0..distances_len) |i| {
        const d = distances[i];
        const max_cluster = uf.merge(d.a, d.b);

        if (max_cluster == points_len) {
            const ans = points[d.a].x * points[d.b].x;
            print("{d}\n", .{ans});
            break;
        }
    }
}
