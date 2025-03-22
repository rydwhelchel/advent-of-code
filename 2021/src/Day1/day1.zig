const std = @import("std");
const data = @embedFile("input");

pub fn part1() !usize {
    var lines = std.mem.tokenizeSequence(u8, data, "\n");

    var prev: usize = std.math.maxInt(usize);
    var count: usize = 0;
    while (lines.next()) |line| {
        const asInt = try std.fmt.parseInt(usize, line, 10);
        if (asInt > prev) {
            count += 1;
        }
        prev = asInt;
    }
    return count;
}

pub fn part2() !void {}
