import random

if __name__ == "__main__":
    digits = "0123456789ABCDEF"
    a_digit1, a_digit2 = random.choice(digits), random.choice(digits)
    value_digit1, value_digit2 = random.choice(digits), random.choice(digits)
    use_carry = random.choice([True, False])
    if use_carry:
        print("SEC")
    else:
        print("CLC")
    print(f"LDA #${a_digit1}{a_digit2}")
    print(f"SBC #${value_digit2}{value_digit2}")
    print("\n\n\n\n")
    print(rf"set_sub_test(&mut cpu, 0x{a_digit1}{a_digit2},"
          rf"0x{value_digit2}{value_digit2}, fillme, fillme, {str(use_carry).lower()})")
