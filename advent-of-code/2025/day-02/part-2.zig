const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var set = std.AutoHashMap(u64, void).init(allocator);
    defer set.deinit();

    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [1000]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var ans: u64 = 0;

    while (try reader.interface.takeDelimiter(',')) |line| {
        const trimmed = std.mem.trim(u8, line, "\n");
        var parts = std.mem.splitSequence(u8, trimmed, "-");

        const beg = try std.fmt.parseInt(u64, parts.next().?, 10);
        const end = try std.fmt.parseInt(u64, parts.next().?, 10);

        // DEBUGGING
        // print("(BEG, END) = ({d}, {d})\n", .{ beg, end });

        set.clearRetainingCapacity();

        for (2..digitsCount(u64, end) + 1) |d| {
            const pattern_beg = getPatternCeil(u64, beg, d);
            const pattern_end = getPatternFloor(u64, end, d);

            // DEBUGGING
            // print("(PATTERN_BEG, PATTERN_END) = ({d}, {d})\n", .{ pattern_beg, pattern_end });

            const invalid_beg = repeatPattern(u64, pattern_beg, d);
            const invalid_end = repeatPattern(u64, pattern_end, d);

            // DEBUGGING
            // print("(INVALID_BEG, INVALID_END) = ({d}, {d})\n", .{ invalid_beg, invalid_end });

            if (invalid_beg <= invalid_end) {
                for (pattern_beg..pattern_end + 1) |pattern| {
                    const invalid_id = repeatPattern(u64, pattern, d);
                    if (!set.contains(invalid_id)) {
                        ans += invalid_id;
                        try set.put(invalid_id, {});
                    }
                }
            } else if (invalid_beg <= end and !set.contains(invalid_beg)) {
                ans += invalid_beg;
                try set.put(invalid_beg, {});
            } else if (invalid_end >= beg and !set.contains(invalid_end)) {
                ans += invalid_end;
                try set.put(invalid_end, {});
            }
        }
    }

    print("{d}\n", .{ans});
}

pub fn pow10(comptime T: type, exponent: T) T {
    return std.math.pow(T, 10, exponent);
}

pub fn digitsCount(comptime T: type, n: T) usize {
    return std.math.log(T, 10, @intCast(n)) + 1;
}

// beg:
//   - large/small make large/large
//     44444/22222 make 44444/44444
//   - small/large make sml+1/sml+1
//     22222/44444 make 22223/22223
pub fn getPatternCeil(comptime T: type, num: T, parts: usize) T {
    const num_d = digitsCount(T, num);

    const pattern_d = @divTrunc(num_d, parts);

    if (@mod(num_d, parts) != 0) {
        return pow10(T, pattern_d);
    }

    var pattern = @mod(num, pow10(T, pattern_d));

    for (1..parts) |p| {
        const pattern_next = @mod(@divTrunc(num, pow10(T, pattern_d * p)), pow10(T, pattern_d));
        pattern = if (pattern_next >= pattern) pattern_next else pattern_next + 1;
    }

    return pattern;
}

// end:
//   - small/large make small/small
//     22222/44444 make 22222/22222
//   - large/small make lrg-1/lrg-1
//     44444/22222 make 44443/44443
pub fn getPatternFloor(comptime T: type, num: T, parts: usize) T {
    const num_d = digitsCount(T, num);

    const pattern_d = @divTrunc(num_d, parts);

    if (@mod(num_d, parts) != 0) {
        return pow10(T, pattern_d) - 1;
    }

    var pattern = @mod(num, pow10(T, pattern_d));

    for (1..parts) |p| {
        const pattern_next = @mod(@divTrunc(num, pow10(T, pattern_d * p)), pow10(T, pattern_d));
        pattern = if (pattern_next <= pattern) pattern_next else pattern_next - 1;
    }

    return pattern;
}

pub fn repeatPattern(comptime T: type, pattern: T, times: usize) T {
    if (pattern == 0) return 0;

    const pattern_d = digitsCount(T, pattern);
    var repeated = pattern;
    for (1..times) |_| {
        repeated = repeated * pow10(T, pattern_d) + pattern;
    }
    return repeated;
}

test {
    try std.testing.expectEqual(12, getPatternCeil(u64, 121212, 3));
    try std.testing.expectEqual(123, getPatternCeil(u64, 123123, 2));
}
test {
    try std.testing.expectEqual(89, getPatternCeil(u64, 889128, 3));
}
test {
    try std.testing.expectEqual(10, getPatternCeil(u64, 21212, 3));
}

test {
    try std.testing.expectEqual(12, getPatternFloor(u64, 121212, 3));
    try std.testing.expectEqual(123, getPatternFloor(u64, 123123, 2));
}
test {
    try std.testing.expectEqual(88, getPatternFloor(u64, 889128, 3));
}
test {
    try std.testing.expectEqual(9, getPatternFloor(u64, 21212, 3));
}

test {
    try std.testing.expectEqual(123123, repeatPattern(u64, 123, 2));
    try std.testing.expectEqual(999, repeatPattern(u64, 9, 3));
}
