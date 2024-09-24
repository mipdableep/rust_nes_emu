import json
import re

traceback = '''pc 1737, opcode 149, args 18 202
pc 1739, opcode 202, args 16 249
pc 1740, opcode 16, args 249 165
pc 1742, opcode 165, args 2 74
pc 1744, opcode 74, args 176 9
pc 1745, opcode 176, args 9 74
pc 1747, opcode 74, args 176 25
pc 1748, opcode 176, args 25 74
pc 1775, opcode 230, args 16 169
pc 1777, opcode 169, args 31 36
pc 1779, opcode 36, args 16 240
pc 1781, opcode 240, args 31 96
pc 1814, opcode 76, args 53 7
pc 1845, opcode 0, args 0 0
pc 0, opcode 10, args 2 2
pc 1, opcode 2, args 2 4'''

class Opcode:
    def __init__(self,
                 bytes: str,
                 description: str,
                 name: str,
                 opcode: str,
                 mode: str):  # Maybe should create enum
        self.bytes = int(bytes)
        self.description = description
        self.name = name
        self.opcode: int = int(opcode, 16)
        self.mode = mode

    def __repr__(self):
        return f'{self.bytes=}, {self.description=}, {self.name=}, {self.opcode=}, {self.mode=}'


with open("./opcodes_6502.json") as f:
    j = f.read()
opcodes_array = [Opcode(**x) for x in json.loads(j)]
opcodes_dict: dict[int, Opcode] = {opcode.opcode: opcode for opcode in opcodes_array}

def translate_rust_traceback(rust_traceback: str):
    line_template = r'pc (\d+), opcode (\d+), args (\d+) (\d+)'
    for line in traceback.split('\n'):
        match = re.match(line_template, line)
        pc, opcode_number, arg1, arg2 = int(match.group(1)), int(match.group(2)), int(match.group(3)), int(match.group(4))
        opcode: Opcode = opcodes_dict[opcode_number]
        if opcode.bytes == 1:
            arg = None
        if opcode.bytes == 2:
            arg = arg1
        if opcode.bytes == 3:
            arg = arg2 * 256 + arg1
        print(f'pc {pc}, opcode {opcode.name} {arg}')

translate_rust_traceback(traceback)