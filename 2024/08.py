from collections import defaultdict
from itertools import combinations


test_ip = """............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"""


def vec_add(a, b):
    return (a[0] + b[0]), (a[1] + b[1])


def vec_sub(a, b):
    return (a[0] - b[0]), (a[1] - b[1])


def on_grid(grid, a):
    return 0 <= a[0] < len(grid) and 0 <= a[1] < len(grid[0])


def part_one(ip):
    grid = [line for line in ip.splitlines()]

    ant = defaultdict(set)

    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c != ".":
                ant[c].add((y, x))

    antinodes = set()
    for ant_set in ant.values():
        for a, b in combinations(ant_set, 2):
            ab = vec_sub(b, a)
            an = vec_sub(a, ab)
            if on_grid(grid, an):
                antinodes.add(an)

            bn = vec_add(b, ab)
            if on_grid(grid, bn):
                antinodes.add(bn)

    return len(antinodes)


test_total = part_one(test_ip)

assert test_total == 14, test_total

inp = """...............e...........j6.....................
.....1...............................t.....i......
.....4.......3..............x..tL......m..........
.......L.....................Dxj..................
4....X..................F.....................m...
.............4.......x....F........k..............
......3...................t..........i.........Z..
....L..................y.....F..e.....Z...........
X.............1........C..........i...D...........
........4.....................D.....k.X...m.......
...1...............D........e......6..............
...3.Y...................................m8.......
..OL.........................x....Z....g..........
....3......5.........................6j...........
...................J..5r.F..k...y.................
.......................................Z..a.......
...........................5........j.........a.u.
...p..............Y....X..........................
...O.........................kd...................
........................t.................i.......
..................J..............u...........z....
.O.....9.............J..............p..u..........
.....9............................................
l...6.....1........e......I................a......
...................................az.............
........M.......J...................gI....z.......
.......Y...l...........p......g....d.......W......
........5l....9................d.....g............
.A....9.l.Y............I..............B.......s...
..................................K.....B.........
....M.............7.......8..........h.....K......
.......0f...oc..............G...d7.......z...s..yW
...M........0...........Gf.....................T..
................r......G..................w....h..
...........cP................G.8.R..............T.
.................A.............N............u..B..
..H.c..b............................K...CB.....y..
......c...bP...2............7..K..................
......b.o....0.......P.............s........h.R...
......2........f..S........8.....................R
U....2..............p..............7..............
.HE..b......A.............N..............w....C...
................................N.............w...
.........E...........M................W.......T...
......E...rS2...........W....................N....
.....SP..n.....r..0...............................
.....H..............A............................w
..........n..U....................s...............
..n.So.....U................f.....................
Ho................................................"""

total = part_one(inp)
print(total)


def part_two(ip):
    grid = [line for line in ip.splitlines()]

    ant = defaultdict(set)

    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c != ".":
                ant[c].add((y, x))

    antinodes = set()
    for ant_set in ant.values():
        for a, b in combinations(ant_set, 2):
            ab = vec_sub(b, a)
            cand_sub = a
            while on_grid(grid, cand_sub):
                antinodes.add(cand_sub)
                cand_sub = vec_sub(cand_sub, ab)

            cand_add = b
            while on_grid(grid, cand_add):
                antinodes.add(cand_add)
                cand_add = vec_add(cand_add, ab)

    return len(antinodes)


test_total = part_two(test_ip)

assert test_total == 34, test_total

total = part_two(inp)
print(total)
