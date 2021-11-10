import 'dart:io';

void main() {
  final intCodes = [
    for (final i in File("input").readAsStringSync().split(",")) int.parse(i),
  ];

  partOne([...intCodes]);
  partTwo([...intCodes]);
}

void partOne(List<int> intCodes) {
  final result = runAndGetInitialValue(
    intCodes: intCodes,
    noun: 12,
    verb: 2,
  );

  print("Part one: $result");
}

void partTwo(List<int> intCodes) {
  for (int noun = 1; noun < 99; noun++) {
    for (int verb = 1; verb < 99; verb++) {
      final result = runAndGetInitialValue(
        intCodes: [...intCodes],
        noun: noun,
        verb: verb,
      );

      if (result == 19690720) {
        print("Part two: ${100 * noun + verb}");
      }
    }
  }
}

int runAndGetInitialValue({
  required List<int> intCodes,
  required int noun,
  required int verb,
}) {
  intCodes[1] = noun;
  intCodes[2] = verb;

  for (int i = 0; i < intCodes.length; i += 4) {
    final operation = intCodes[i];

    if (operation == 1) {
      intCodes[intCodes[i + 3]] = intCodes[intCodes[i + 1]] + intCodes[intCodes[i + 2]];
    } else if (operation == 2) {
      intCodes[intCodes[i + 3]] = intCodes[intCodes[i + 1]] * intCodes[intCodes[i + 2]];
    } else if (operation == 99) {
      break;
    } else {
      print("Unknown operation: $operation");
      exit(1);
    }
  }

  return intCodes[0];
}
