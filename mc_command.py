import keyboard

filename = input("Enter binary filename.\n> ")
with open(filename, "r") as exe:
    lines = exe.readlines()
exe = [x.strip("\n") for x in lines]

commands = []

for idx, instruction in enumerate(exe):
    x_pos = idx * 3 + 50
    for i, b in enumerate(instruction):
        y_pos = i * 2 + 101
        block = "air"
        if b == "1": block = "redstone_block"
        commands.append(f"/setblock {x_pos} {y_pos} {60} {block}")
