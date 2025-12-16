const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

const MAX_LINE_LEN = 1000;
const MAX_LIGHTS = u64;
const MAX_BUTTONS = u64;

pub fn main() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [MAX_LINE_LEN]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var total_presses: usize = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');

        var lights_target_state: MAX_LIGHTS = 0;

        var buttons: [@typeInfo(MAX_BUTTONS).int.bits]MAX_LIGHTS = undefined;
        var buttons_len: usize = 0;

        while (it.next()) |s| {
            if (s[0] == '[') {
                for (1..s.len - 1) |i| {
                    if (s[i] == '#') lights_target_state |= @as(MAX_LIGHTS, @intCast(1)) << @intCast(i - 1);
                }
            }
            if (s[0] == '(') {
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
                break;
            }
        }

        var min_presses: usize = buttons_len;

        for (0..(@as(MAX_LIGHTS, @intCast(1)) << @intCast(buttons_len))) |selection| {
            var light: MAX_LIGHTS = 0;
            for (0..buttons_len) |b| {
                if (selection & (@as(MAX_LIGHTS, @intCast(1)) << @intCast(b)) > 0) light ^= buttons[b];
            }
            if (light == lights_target_state and @popCount(selection) < min_presses) {
                min_presses = @popCount(selection);
            }
        }

        total_presses += min_presses;
    }

    print("{d}\n", .{total_presses});
}
