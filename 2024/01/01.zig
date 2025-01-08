const data = @embedFile("input.txt");
const std = @import("std");
const mem = std.mem;
const debug = std.debug;
const ArrayList = std.ArrayList;
const parseInt = std.fmt.parseInt;
const asc_i32 = std.sort.asc(i32);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var list_a = ArrayList(i32).init(allocator);
    defer list_a.deinit();
    var list_b = ArrayList(i32).init(allocator);
    defer list_b.deinit();

    var it_line = mem.tokenizeAny(u8, data, "\r\n");
    while (it_line.next()) |line| {
        std.debug.print("{s}\n", .{line});
        var it_loc = std.mem.tokenizeScalar(u8, line, ' ');
        const loc_a = parseInt(i32, it_loc.next().?, 10) catch unreachable;
        try list_a.append(loc_a);
        const loc_b = parseInt(i32, it_loc.next().?, 10) catch unreachable;
        try list_b.append(loc_b);
    }
    mem.sort(i32, list_a.items, {}, asc_i32);
    mem.sort(i32, list_b.items, {}, asc_i32);

    var total: i32 = 0;
    for (list_a.items, 0..) |loc_a, i| {
        total += @intCast(@abs(loc_a - list_b.items[i]));
    }
    std.debug.print("{d}\n", .{total});
}
