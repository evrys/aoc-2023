import re
from dataclasses import dataclass


@dataclass
class EngineNumber:
  number: int
  y: int
  x1: int
  x2: int


@dataclass
class EngineSymbol:
  ch: str
  y: int
  x: int


def is_adjacent(number: EngineNumber, symbol: EngineSymbol):
  for x in range(number.x1, number.x2+1):
    if abs(x - symbol.x) <= 1 and abs(number.y - symbol.y) <= 1:
      return True
  return False


def parse_input(lines: list[str]):
  numbers = []
  symbols = []

  for y, line in enumerate(lines):
    for m in re.finditer(r'\d+', line):
      numbers.append(EngineNumber(int(m[0]), y, m.start(), m.end()-1))
    for m in re.finditer(r'[^0-9\.\s]', line):
      symbols.append(EngineSymbol(m[0], y, m.start()))

  return numbers, symbols


def part1(lines: list[str]):
  numbers, symbols = parse_input(lines)

  total = 0
  for num in numbers:
    if any(is_adjacent(num, sym) for sym in symbols):
      total += num.number
  
  print("Part 1:", total)


def part2(lines: list[str]):
  numbers, symbols = parse_input(lines)

  total = 0
  for sym in symbols:
    if sym.ch != '*':
      continue
    adjacent_numbers = [num for num in numbers if is_adjacent(num, sym)]
    if len(adjacent_numbers) == 2:
      total += adjacent_numbers[0].number * adjacent_numbers[1].number
  
  print("Part 2:", total)


if __name__ == "__main__":
  with open('input.txt') as f:
    lines = f.readlines()
    part1(lines)
    part2(lines)