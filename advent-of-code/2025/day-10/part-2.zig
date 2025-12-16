const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_LINE_LEN = 1000;
const MAX_BUTTONS = 20;
const MAX_JOLTAGES = 20;
const MAX_LIGHTS = u64;
const MAX_ALLOC = 1000000;

pub fn solve(
    cache: std.AutoHashMap(MAX_LIGHTS, *std.ArrayList(MAX_LIGHTS)),
    joltages: []u64,
    buttons: []MAX_LIGHTS,
    depth: usize,
) u64 {
    for (0..depth) |_| std.debug.print("  ", .{});
    print("Solve:\n", .{});
    var light_state: MAX_LIGHTS = 0;
    var total: usize = 0;
    for (0..depth) |_| std.debug.print("  ", .{});
    for (0..joltages.len) |i| {
        print("{d} ", .{joltages[i]});
        total += joltages[i];
        if (joltages[i] % 2 == 1) light_state |= @as(MAX_LIGHTS, @intCast(1)) << @intCast(i);
    }
    print("\n", .{});

    if (total == 0) return 0;

    for (0..depth) |_| std.debug.print("  ", .{});
    print("Pattern {b}\n", .{light_state});

    if (!cache.contains(light_state)) return std.math.maxInt(u64);
    print("YES CACHE\n", .{});

    const list = cache.get(light_state).?;

    var min_cost: u64 = std.math.maxInt(u64);
    outer: for (list.items) |item| {
        for (0..depth) |_| std.debug.print("  ", .{});
        print("Selection: {b}\n", .{item});
        var joltages_next: [MAX_JOLTAGES]u64 = undefined;
        std.mem.copyForwards(u64, &joltages_next, joltages);
        for (0..buttons.len) |btn| {
            const is_active = item & (@as(MAX_LIGHTS, @intCast(1)) << @intCast(btn)) > 0;
            if (!is_active) continue;
            for (0..depth) |_| std.debug.print("  ", .{});
            print("Button: {b}\n", .{buttons[btn]});
            for (0..@typeInfo(MAX_LIGHTS).int.bits) |j| {
                if (buttons[btn] & (@as(MAX_LIGHTS, @intCast(1)) << @intCast(j)) > 0) {
                    if (joltages_next[j] == 0) continue :outer;
                    joltages_next[j] -= 1;
                }
            }
        }
        for (0..joltages.len) |j| {
            joltages_next[j] /= 2;
        }

        for (0..depth) |_| std.debug.print("  ", .{});
        print("Sub\n", .{});
        const cost_next = solve(cache, joltages_next[0..joltages.len], buttons, depth + 1);
        for (0..depth) |_| std.debug.print("  ", .{});
        print("Sub Done\n", .{});
        if (cost_next == std.math.maxInt(u64)) continue;
        const cost = @popCount(item) + 2 * cost_next;
        if (cost < min_cost) {
            min_cost = cost;
            for (0..depth) |_| std.debug.print("  ", .{});
            print("Minimal Selection With Cost {d}\n", .{cost});
        }
    }

    for (0..depth) |_| std.debug.print("  ", .{});
    print("Return\n", .{});

    return min_cost;
}

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [MAX_LINE_LEN]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var total_presses: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');

        var buttons: [MAX_BUTTONS]MAX_LIGHTS = undefined;
        var buttons_len: usize = 0;

        var joltages: [MAX_JOLTAGES]u64 = undefined;
        var joltages_len: usize = 0;

        while (it.next()) |s| {
            if (s[0] == '[') {
                // Lights
                continue;
            }
            if (s[0] == '(') {
                // Buttons
                var it_button = std.mem.splitScalar(u8, s[1 .. s.len - 1], ',');
                var button: MAX_LIGHTS = 0;
                while (it_button.next()) |l| {
                    const l_idx = try std.fmt.parseInt(MAX_LIGHTS, l, 10);
                    button |= @as(MAX_LIGHTS, @intCast(1)) << @intCast(l_idx);
                }
                buttons[buttons_len] = button;
                buttons_len += 1;
            }
            if (s[0] == '{') {
                // Joltage
                var it_joltage = std.mem.splitScalar(u8, s[1 .. s.len - 1], ',');
                while (it_joltage.next()) |j_str| {
                    const j = try std.fmt.parseInt(u64, j_str, 10);
                    joltages[joltages_len] = j;
                    joltages_len += 1;
                }
            }
        }

        var alloc_buffer: [MAX_ALLOC]u8 = undefined;
        var fba = std.heap.FixedBufferAllocator.init(&alloc_buffer);
        const allocator = fba.allocator();

        var cache = std.AutoHashMap(MAX_LIGHTS, *std.ArrayList(MAX_LIGHTS)).init(
            allocator,
        );

        // Fill Cache
        for (0..(@as(MAX_LIGHTS, @intCast(1)) << @intCast(buttons_len))) |selection| {
            var light: MAX_LIGHTS = 0;
            for (0..buttons_len) |b| {
                if (selection & (@as(MAX_LIGHTS, @intCast(1)) << @intCast(b)) > 0) light ^= buttons[b];
            }
            if (!cache.contains(light)) {
                const list = try allocator.create(std.ArrayList(MAX_LIGHTS));
                list.* = std.ArrayList(MAX_LIGHTS).empty;
                try cache.put(light, list);
            }
            print("LIG {b} SEL {b}\n", .{ light, selection });
            try cache.get(light).?.append(allocator, selection);
        }

        const min_presses = solve(cache, joltages[0..joltages_len], buttons[0..buttons_len], 0);
        print("Min Presses: {d}\n", .{min_presses});
        total_presses += min_presses;
    }

    print("{d}\n", .{total_presses});
}
