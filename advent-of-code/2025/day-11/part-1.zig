const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

// Device names are three-letter ASCII strings
const MAX_DEVICES = 28 * 28 * 28;
const MAX_CONNS = 30;

const Device = struct {
    conns: [MAX_CONNS]u16,
    conns_len: usize,
};

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var devices: [MAX_DEVICES]Device = undefined;
    const you_idx = device_name_to_idx("you");
    const out_idx = device_name_to_idx("out");

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const name = line[0..3];
        const idx = device_name_to_idx(name);

        var device = Device{ .conns = undefined, .conns_len = 0 };

        var it = std.mem.splitScalar(u8, line[5..], ' ');
        while (it.next()) |conn_name| {
            const conn_idx = device_name_to_idx(conn_name[0..3]);
            device.conns[device.conns_len] = conn_idx;
            device.conns_len += 1;
        }

        devices[idx] = device;
    }

    const paths = count_paths(&devices, you_idx, out_idx);
    print("{d}\n", .{paths});
}

pub fn device_name_to_idx(name: *const [3]u8) u16 {
    const a: u16 = @intCast(name[0] - 'a');
    const b: u16 = @intCast(name[1] - 'a');
    const c: u16 = @intCast(name[2] - 'a');
    return (28 * 28) * a + 28 * b + c;
}

pub fn count_paths(devices: []Device, cur_idx: u16, out_idx: u16) u64 {
    if (cur_idx == out_idx) return 1;

    var paths: u64 = 0;
    for (0..devices[cur_idx].conns_len) |i| {
        const nxt_idx = devices[cur_idx].conns[i];
        paths += count_paths(devices, @intCast(nxt_idx), out_idx);
    }
    return paths;
}
