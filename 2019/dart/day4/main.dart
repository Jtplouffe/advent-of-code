const min = 402328, max = 864247;

void main() {
  print("Part one: ${getPartOneValidPasswords().length}");
  print("Part two: ${getPartTwoValidPasswords().length}");
}

List<int> getPartOneValidPasswords() {
  return [
    for (int p = min; p <= max; p++)
      if (p.anyTwoSameAdjacentDigit() && p.everyDigitNotDecreasing()) p,
  ];
}

List<int> getPartTwoValidPasswords() {
  return [
    for (int p = min; p <= max; p++)
      if (p.anyExactlyTwoSameAdjacentDigit() && p.everyDigitNotDecreasing()) p,
  ];
}

extension IntPasswordValiadionExt on int {
  bool anyTwoSameAdjacentDigit() {
    int remaining = this ~/ 10;
    int lastDigit = this % 10;
    while (remaining > 0) {
      final currentDigit = remaining % 10;
      if (currentDigit == lastDigit) return true;
      remaining ~/= 10;
      lastDigit = currentDigit;
    }

    return false;
  }

  bool anyExactlyTwoSameAdjacentDigit() {
    int remaining = this ~/ 10;
    int lastDigit = this % 10;
    while (remaining > 0) {
      final currentDigit = remaining % 10;
      if (currentDigit == lastDigit) {
        if (!RegExp(r"(" + currentDigit.toString() + r")\1\1").hasMatch(this.toString())) {
          return true;
        }
      }
      remaining ~/= 10;
      lastDigit = currentDigit;
    }

    return false;
  }

  bool everyDigitNotDecreasing() {
    int remaining = this ~/ 10;
    int lastDigit = this % 10;
    while (remaining > 0) {
      final currentDigit = remaining % 10;
      if (currentDigit > lastDigit) return false;
      remaining ~/= 10;
      lastDigit = currentDigit;
    }

    return true;
  }
}
