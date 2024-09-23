import json

arithmatic_logic = [
    "ADC",
    "AND",
    "ASL",
    "BIT",
    "CMP",
    "DEC",
    "EOR",
    "LSR",
    "ORA",
    "ROL",
    "ROR",
    "SBC",
]

control_flow = [
    "BCC",
    "BCS",
    "BEQ",
    "BMI",
    "BNE",
    "BPL",
    "BVC",
    "BVS",
    "JMP",
    "JSR",
    "RTS",
]

stack_related = [
    "PHA",
    "PHP",
    "PLA",
    "PLP",
]

status_register = [
    "CLC",
    "CLD",
    "CLI",
    "CLV",
    "SEC",
    "SED",
    "SEI",
]

interrupts = [
    "RTI",
    "BRK",
]
no_op = ["NOP"]

register_ops = [
    "CPX",
    "CPY",
    "DEX",
    "DEY",
    "INC",
    "INX",
    "INY",
    "LDA",
    "LDX",
    "LDY",
    "STA",
    "STX",
    "STY",
    "TAX",
    "TAY",
    "TSX",
    "TXA",
    "TXS",
    "TYA",
]


def cat_printer(cat):
    s = ""
    if cat == register_ops:
        s = "register_ops"
    if cat == interrupts:
        s = "interrupts"
    if cat == no_op:
        s = "no_op"
    if cat == arithmatic_logic:
        s = "arithmatic_logic"
    if cat == control_flow:
        s = "control_flow"
    if cat == stack_related:
        s = "stack_related"
    if cat == status_register:
        s = "status_register"

    padded_s = f"//// {s} /////"
    breaker = "/" * len(padded_s)
    print("\n", breaker, sep="")
    print(padded_s)
    print(breaker, "\n", sep="")


catagories = [
    register_ops,
    interrupts,
    no_op,
    arithmatic_logic,
    control_flow,
    stack_related,
    status_register,
]
with open("./opcodes_6502.json") as f:
    j = f.read()
j = json.loads(j)

# for op in control_flow:
#     op_desc = set()
#     op_name = ""
#     op_name = op
#     for op_j in j:
#         if op_j["name"] == op:
#             op_desc.add(op_j["description"])
#
#     if len(op_desc) == 1:
#         print("/// ", list(op_desc)[0])
#     else:
#         print(op_desc)
#     print(f"pub fn {op_name}(&mut self) ", "{ todo!() }")
#     print("")

for cat in catagories:
    cat_printer(cat)
    for op in cat:
        op_desc = ""
        op_name = op

        # get description and print
        for op_j in j:
            if op_j["name"] == op:
                op_desc = op_j["description"]
                print("\n//", op_name, ":", op_desc)
                break

        for op_j in j:
            if op_j["name"] == op:
                opcode = op_j["opcode"]
                addressing_mode = op_j["mode"]
                bytes_number = int(op_j["bytes"])
                print(f"{opcode} => {{",
                      f"self.program_counter += 1;",
                      f"let addressing_mode = AddressingMode::{addressing_mode};",
                      f"let value = self.convert_mode_to_val(addressing_mode);",
                      f"self.program_counter += {bytes_number-1};",
                      f"self.{op_name}(value);",
                      # this is true for most operations, but if we need the address the LSP will give us the nice
                      # red line, and we will fix it manually
                      f"}}", sep="\n")
