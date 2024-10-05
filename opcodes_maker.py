# This file takes the opcodes_with_cycles json from https://github.com/ericTheEchidna/65C02-JSON/blob/main/opcodes_65c02.json
# and formats it to be more usable

import json
import pydantic
from pydantic import BaseModel


class Instruction(BaseModel):
    opcode: str
    bytes: int
    base_cycles: int
    increase_on_page_cross: bool
    is_branch: bool
    address_mode: str


x = {'address_mode': 'Immediate',
     'bytes': '2',
     'cycles': '2 (+1 if branch taken, +2 if to a new page)'}
with open("./opcodes_with_cycles.json") as f:
    instruction_set = json.loads(f.read())

with open("./opcodes_6502.json") as f:
    instruction_set_without_cycles = json.loads(f.read())

our_instruction_list = []

for instruction in instruction_set:
    for opcode, opcode_metadata in instruction["opcodes"].items():
        list_of_matching_instructions = [instruction_set_without_cycles[i] for i in
                                         range(len(instruction_set_without_cycles)) if
                                         instruction_set_without_cycles[i]["opcode"].lower() == opcode.lower()]
        if not list_of_matching_instructions:
            continue
        if len(list_of_matching_instructions) != 1:
            print("Found more than one matching opcode")
            exit(1)
        address_mode = list_of_matching_instructions[0]["mode"]

        instruction_bytes = int(opcode_metadata["bytes"])
        base_cycles = int(opcode_metadata["cycles"].split(" ")[0])
        is_branch = "branch" in instruction["description"].lower()
        if is_branch:
            increase_on_page_cross = True  # always true for branch
        else:
            increase_on_page_cross = "+1" in opcode_metadata["cycles"].lower()

        our_instruction_list.append(Instruction(opcode=opcode, bytes=instruction_bytes, base_cycles=base_cycles,
                                                increase_on_page_cross=increase_on_page_cross, is_branch=is_branch,
                                                address_mode=address_mode))

        print(our_instruction_list[-1])

assert (len(our_instruction_list) == len(instruction_set_without_cycles))
