import 'dart:io';

import 'int_code_instruction.dart';

void main() {
  final intCodes = File("input").readAsStringSync().split(",").map(int.parse).toList();

  final interpreter = IntCodesInterpreter(intCodes: intCodes, input: 5);
  interpreter.run();
}

class IntCodesInterpreter {
  final List<int> intCodes;
  int input;
  int ip;

  IntCodesInterpreter({
    required this.intCodes,
    required this.input,
  }) : ip = 0;

  void run() {
    while (true) {
      ip = IntCodeInstruction(ip: ip, intCodes: intCodes, input: input).process();
    }
  }
}
