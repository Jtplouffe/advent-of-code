import 'dart:io';

void main() async {
  final input = await File("./input").readAsLines();

  runPartOne(input);
  runPartTwo(input);
}

int calculateString(String line) => calculateInt(int.parse(line));

int calculateInt(int value) => value ~/ 3 - 2;

void runPartOne(List<String> input) {
  print(input.map(calculateString).sum());
}

void runPartTwo(List<String> input) {
  final masses = <int>[];
  for (final line in input) {
    masses.add(calculateString(line));

    do {
      masses.add(calculateInt(masses.last));
    } while (masses.last > 0);
  }

  print(masses.where((m) => m > 0).sum());
}

extension IterableIntExt on Iterable<int> {
  int sum() => reduce((value, element) => value + element);
}
