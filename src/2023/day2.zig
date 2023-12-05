const std = @import("std");
const mem = std.mem;
const fs = std.fs;
const fmt = std.fmt;
const Allocator = mem.Allocator;
const ArrayList = std.ArrayList;
const dbg = std.debug.print;

var gpa_instance = std.heap.GeneralPurposeAllocator(.{}){};
const gpa = gpa_instance.allocator();

pub fn main() !void {
    const input = try readFileToArrayList(gpa, "src/2023/day2.txt");
    defer input.deinit();

    const games = try parse_games(input.items);
    defer {
        for (games.items) |game| game.deinit();
        games.deinit();
    }

    var possible_games_sum: u32 = 0;
    const max_step = Step{ .red = 12, .green = 13, .blue = 14 };

    outer: for (games.items) |game| {
        for (game.steps.items) |step| {
            if ((step.red > max_step.red) or (step.green > max_step.green) or (step.blue > max_step.blue))
                continue :outer;
        }
        possible_games_sum += game.id;
        // dbg("Game {{\n  id: {},\n  steps: {any}\n}}\n", .{ game.id, game.steps.items });
    }

    dbg("Part 1: {}\n", .{possible_games_sum});

    var power_sum: u32 = 0;
    for (games.items) |game| {
        var min_step = Step{ .red = 0, .green = 0, .blue = 0 };

        for (game.steps.items) |step| {
            min_step.red = @max(min_step.red, step.red);
            min_step.green = @max(min_step.green, step.green);
            min_step.blue = @max(min_step.blue, step.blue);
        }

        power_sum += min_step.red * min_step.blue * min_step.green;
    }

    dbg("Part 2: {}\n", .{power_sum});
}

fn parse_games(input: []const u8) !ArrayList(Game) {
    var games = ArrayList(Game).init(gpa);
    var lines = mem.split(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) continue;
        try games.append(try Game.from_str(gpa, line));
    }
    return games;
}

fn readFileToArrayList(alloc: Allocator, path: []const u8) !ArrayList(u8) {
    const file = try fs.cwd().openFile(path, .{ .mode = .read_only });
    defer file.close();

    var input = ArrayList(u8).init(alloc);

    try file.reader().readAllArrayList(&input, std.math.maxInt(usize));
    return input;
}

const Game = struct {
    id: u32,
    steps: ArrayList(Step),

    const ParseGameError = error{
        InvalidFormat,
        Overflow,
        InvalidCharacter,
        OutOfMemory,
    };

    fn from_str(alloc: Allocator, str: []const u8) ParseGameError!Game {
        const colon_pos = mem.indexOf(u8, str, ":") orelse {
            return ParseGameError.InvalidFormat;
        };
        const id = try fmt.parseInt(u32, str[5..colon_pos], 10);

        var game = try Game.init(alloc, id);

        const steps_str = str[colon_pos + 1 ..];
        var steps = mem.split(u8, steps_str, ";");
        while (steps.next()) |step_str| {
            try game.steps.append(try Step.from_str(step_str));
        }

        return game;
    }

    fn init(alloc: Allocator, id: u32) error{OutOfMemory}!Game {
        return Game{ .id = id, .steps = ArrayList(Step).init(alloc) };
    }

    fn deinit(self: Game) void {
        self.steps.deinit();
    }
};

const Step = struct {
    red: u32,
    green: u32,
    blue: u32,

    /// Parse a Step from a string with format
    ///
    /// ```
    /// step ::= cube ("," cube)*
    /// cube ::= " " int " " ("red" | "green" | "blue")
    /// ```
    fn from_str(str: []const u8) fmt.ParseIntError!Step {
        var step = Step{ .red = 0, .green = 0, .blue = 0 };

        var cubes = mem.split(u8, str, ",");
        while (cubes.next()) |padded_str| {
            // Discard leading space
            const cube_str = padded_str[1..];
            const space_pos = mem.indexOf(u8, cube_str, " ") orelse continue;

            const name = cube_str[(space_pos + 1)..];
            const amount = try fmt.parseInt(u32, cube_str[0..space_pos], 10);

            if (mem.eql(u8, name, "red")) {
                step.red = amount;
            } else if (mem.eql(u8, name, "green")) {
                step.green = amount;
            } else if (mem.eql(u8, name, "blue")) {
                step.blue = amount;
            } else {
                dbg("Invalid cube name \"{s}\"", .{name});
            }
        }

        return step;
    }
};
