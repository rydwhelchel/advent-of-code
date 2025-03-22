const std = @import("std");
const day1 = @import("day1/day1.zig");

pub fn main() !void {
    // std.debug.print("D1 Part1: {}", day1.part1());
    // std.debug.print("D1 Part2: {}", day1.part2());
    std.debug.print("{d}\n", .{try day1.part1()});
}
