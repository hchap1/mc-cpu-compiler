import keyboard
from time import sleep

filename = input("Enter binary filename.\n> ")
with open(filename, "r") as exe:
    lines = exe.readlines()
exe = [x.strip("\n") for x in lines]

commands = []

for idx, instruction in enumerate(exe):
    x_pos = idx * -3 + 58
    for i, b in enumerate(instruction):
        y_pos = 115 - i * 2
        block = "air"
        if b == "1": block = "redstone_block"
        commands.append(f"setblock {x_pos} {y_pos} {60} {block}")

while not keyboard.is_pressed("`"):
    sleep(0.05)
while keyboard.is_pressed("`"):
    sleep(0.05)
for command in commands:
    keyboard.press("/")
    sleep(0.02)
    keyboard.release("/")
    sleep(0.05)
    keyboard.write(command)
    sleep(0.05)
    keyboard.press("enter")
    sleep(0.02)
    keyboard.release("enter")
    sleep(0.05)
