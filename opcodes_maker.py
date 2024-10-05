# This file takes the opcodes_with_cycles json from https://gist.github.com/benhess02/3873f6e5f305cd22ee5e1067d080c016
# and formats it to be more usable

import json
from pydantic import BaseModel


class Instruction(BaseModel):
    opcode: str
    bytes: int
    base_cycles: int
    increase_on_page_cross: bool
    address_mode: str

    def to_string_rust_format(self) -> str:
        return (f"{self.opcode}_u8 => OpcodeMetadata{{opcode: {self.opcode}, bytes: {self.bytes}, "
                f"base_cycles: {self.base_cycles}, addressing_mode: AddressingMode::{self.address_mode}, "
                f"increase_on_page_cross: {str(self.increase_on_page_cross).lower()} }}")


x = {'address_mode': 'Immediate',
     'bytes': '2',
     'cycles': '2 (+1 if branch taken, +2 if to a new page)'}
with open("./opcodes_with_cycles.json") as f:
    instruction_set = json.loads(f.read())

with open("./opcodes_6502.json") as f:
    instruction_set_without_cycles = json.loads(f.read())

our_instruction_list = []

for instruction in instruction_set:
    if bool(instruction["illegal"]):
        continue
    opcode = '0x{:02X}'.format(instruction["opcode"])
    list_of_matching_instructions = [instruction_set_without_cycles[i] for i in
                                     range(len(instruction_set_without_cycles)) if
                                     instruction_set_without_cycles[i]["opcode"].lower() == opcode.lower()]
    if not list_of_matching_instructions:
        continue
    if len(list_of_matching_instructions) != 1:
        print("Found more than one matching opcode")
        exit(1)
    address_mode = list_of_matching_instructions[0]["mode"]
    instruction_bytes = list_of_matching_instructions[0]["bytes"]
    base_cycles = int(instruction["cycles"])
    is_branch = False
    increase_on_page_cross = instruction["pageBoundaryCycle"]
    our_instruction_list.append(Instruction(opcode=opcode, bytes=instruction_bytes, base_cycles=base_cycles,
                                            increase_on_page_cross=increase_on_page_cross, is_branch=is_branch,
                                            address_mode=address_mode))

assert (len(our_instruction_list) == len(instruction_set_without_cycles))
print("pub const OPCODES_METADATA: phf::Map<u8, OpcodeMetadata> = phf_map! {")
for instruction in our_instruction_list:
    print(instruction.to_string_rust_format() + ",")
print("};")
