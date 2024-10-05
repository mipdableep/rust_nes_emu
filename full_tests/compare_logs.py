import os
import re
import subprocess

from pydantic import BaseModel


class Line(BaseModel):
    pc: int
    opcode: int
    a: int
    x: int
    y: int
    sp: int
    p: int


def read_our(our_lines, i):
    our_line = our_lines[i].strip("\n")
    our_line_template = (r' pc: (0x[a-f\d]+) opcode: (0x[a-f\d]+) A: (0x[a-f\d]+), X: (0x[a-f\d]+), '
                         r'Y: (0x[a-f\d]+), SP: (0x[a-f\d]+), Status: (0x[a-f\d]+)')
    match = re.match(our_line_template, our_line)

    pc, opcode, a, x, y, sp, p = (int(match.group(1), 16), int(match.group(2), 16), int(match.group(3), 16),
                                  int(match.group(4), 16), int(match.group(5), 16), int(match.group(6), 16),
                                  int(match.group(7), 16))

    return Line(pc=pc, opcode=opcode, x=x, y=y, a=a, sp=sp, p=p)


def read_their(their_lines, i):
    their_line = their_lines[i].strip("\n")
    their_template = r'([A-F\d]{4})  ([A-F\d]{2}) (.{2}) (.{2})  ([A-Z]{3}) (.{27}) A:([A-F\d]{2}) X:([A-F\d]{2}) Y:([A-F\d]{2}) P:([A-F\d]{2}) SP:([A-F\d]{2}) PPU:(.{7}) CYC:(\d*)'
    match = re.match(their_template, their_line)

    pc = int(match.group(1), 16)
    opcode = int(match.group(2), 16)
    a = int(match.group(7), 16)
    x = int(match.group(8), 16)
    y = int(match.group(9), 16)
    p = int(match.group(10), 16)
    sp = int(match.group(11), 16)
    return Line(pc=pc, opcode=opcode, x=x, y=y, a=a, sp=sp, p=p)


if __name__ == "__main__":
    cwd = os.path.dirname(os.path.realpath(__file__))
    results_file_path = os.path.join(cwd, "our_result.txt")
    test_file = os.path.join(cwd, "nestest.nes")
    proc = subprocess.run(["cargo", "run", "--bin", "gen_cpu_tests_logs", "--", test_file, results_file_path])
    if proc.returncode != 0:
        print(
            "Cargo generation of cpu tests did not end successfully. Probably just the unimplemented unofiicial opcodes")
    with open(results_file_path) as f:
        our_lines = f.readlines()
    with open(os.path.join(cwd, "nestest_result_good.log")) as f:
        their_lines = f.readlines()

    for i in range(5003):
        if read_our(our_lines, i) == read_their(their_lines, i):
            if i % 256 == 0:
                print(f"line {i} passed!")
        else:
            print(f"line {i} failed!")
            print(f"{read_our(our_lines, i)=}")
            print(f"{read_their(their_lines, i)=}")
            exit(1)
