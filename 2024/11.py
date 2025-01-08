test_ip = "125 17"


def part_one(ip):
    stones = [int(s) for s in ip.split()]
    for _ in range(25):
        i = 0
        while i < len(stones):
            stone = stones[i]
            stone_str = str(stone)

            if stone == 0:
                stones[i] = 1
                i += 1
            elif len(stone_str) % 2 == 0:
                halfway = int(len(stone_str) / 2)
                stones[i] = int(stone_str[:halfway])
                stones.insert(i + 1, int(stone_str[halfway:]))
                i += 2
            else:
                stones[i] = stone * 2024
                i += 1

    return len(stones)


test_total = part_one(test_ip)
assert test_total == 55312

inp = "4022724 951333 0 21633 5857 97 702 6"
total = part_one(inp)
print(total)
