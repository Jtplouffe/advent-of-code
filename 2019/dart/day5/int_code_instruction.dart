import 'dart:io';

abstract class IntCodeInstruction {
  int get firstParamValue {
    if (intCodes[ip] % 1000 ~/ 100 == 1) {
      return intCodes[ip + 1];
    }

    return intCodes[intCodes[ip + 1]];
  }

  int get secondParamValue {
    if (intCodes[ip] % 10000 ~/ 1000 == 1) {
      return intCodes[ip + 2];
    }

    return intCodes[intCodes[ip + 2]];
  }

  final int ip;
  final List<int> intCodes;
  final int input;

  const IntCodeInstruction._({
    required this.ip,
    required this.intCodes,
    required this.input,
  });

  factory IntCodeInstruction({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) {
    final instruction = intCodes[ip] % 100;
    switch (instruction) {
      case 1:
        return IntCodeAddInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 2:
        return IntCodeMultiplyInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 3:
        return IntCodeSaveInputInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 4:
        return IntCodeOutputInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 5:
        return IntCodeJumpIfTrueInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 6:
        return IntCodeJumpIfFalseInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 7:
        return IntCodeLessThanInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 8:
        return IntCodeEqualsTrueInstruction._(ip: ip, intCodes: intCodes, input: input);
      case 99:
        exit(0);
      default:
        throw Exception("Unknown instruction: $instruction");
    }
  }

  int process();
}

class IntCodeAddInstruction extends IntCodeInstruction {
  const IntCodeAddInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    intCodes[intCodes[ip + 3]] = firstParamValue + secondParamValue;
    return ip + 4;
  }
}

class IntCodeMultiplyInstruction extends IntCodeInstruction {
  const IntCodeMultiplyInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    intCodes[intCodes[ip + 3]] = firstParamValue * secondParamValue;
    return ip + 4;
  }
}

class IntCodeSaveInputInstruction extends IntCodeInstruction {
  const IntCodeSaveInputInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    intCodes[intCodes[ip + 1]] = input;
    return ip + 2;
  }
}

class IntCodeOutputInstruction extends IntCodeInstruction {
  const IntCodeOutputInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    print(intCodes[intCodes[ip + 1]]);
    return ip + 2;
  }
}

class IntCodeJumpIfTrueInstruction extends IntCodeInstruction {
  const IntCodeJumpIfTrueInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    if (firstParamValue != 0) {
      return secondParamValue;
    }

    return ip + 3;
  }
}

class IntCodeJumpIfFalseInstruction extends IntCodeInstruction {
  const IntCodeJumpIfFalseInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    if (firstParamValue == 0) {
      return secondParamValue;
    }

    return ip + 3;
  }
}

class IntCodeLessThanInstruction extends IntCodeInstruction {
  const IntCodeLessThanInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    if (firstParamValue < secondParamValue) {
      intCodes[intCodes[ip + 3]] = 1;
    } else {
      intCodes[intCodes[ip + 3]] = 0;
    }

    return ip + 4;
  }
}

class IntCodeEqualsTrueInstruction extends IntCodeInstruction {
  const IntCodeEqualsTrueInstruction._({
    required int ip,
    required List<int> intCodes,
    required int input,
  }) : super._(ip: ip, intCodes: intCodes, input: input);

  @override
  int process() {
    if (firstParamValue == secondParamValue) {
      intCodes[intCodes[ip + 3]] = 1;
    } else {
      intCodes[intCodes[ip + 3]] = 0;
    }

    return ip + 4;
  }
}
