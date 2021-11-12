import 'dart:io';
import 'dart:math';

void main() {
  final wires = File("input").readAsStringSync().split("\n");
  final firstWire = wires[0].split(",");
  final secondWire = wires[1].split(",");

  final firstWirePositions = getWirePositions(firstWire);
  final secondWirePositions = getWirePositions(secondWire);

  final intersections = getIntersections(firstWirePositions, secondWirePositions);

  final shortestDistance = getShortestDistance(intersections);
  print("Part one: $shortestDistance");

  final shortestCombinedLengthIntersection = getShortestCombinedLengthIntersection(
    intersections: intersections,
    firstWirePositions: firstWirePositions,
    secondWirePositions: secondWirePositions,
  );
  print("Part two: $shortestCombinedLengthIntersection");
}

Set<Point<int>> getWirePositions(List<String> moves) {
  final positions = <Point<int>>{};

  int x = 0, y = 0;

  for (final move in moves) {
    final direction = move[0];
    final length = move.substring(1).toInt();

    switch (direction) {
      case "L":
        positions.addAll([
          for (int i = x; i >= x - length; i--) Point(i, y),
        ]);

        x -= length;
        break;
      case "U":
        positions.addAll([
          for (int i = y; i <= y + length; i++) Point(x, i),
        ]);

        y += length;
        break;
      case "R":
        positions.addAll([
          for (int i = x; i <= x + length; i++) Point(i, y),
        ]);

        x += length;
        break;
      case "D":
        positions.addAll([
          for (int i = y; i >= y - length; i--) Point(x, i),
        ]);

        y -= length;
        break;
      default:
        throw Exception("Direction not found: $direction");
    }
  }

  positions.removeWhere((p) => p.x == 0 && p.y == 0);

  return positions;
}

Iterable<Point<int>> getIntersections(Iterable<Point<int>> firstPositions, Iterable<Point<int>> secondPositions) {
  return {
    for (final pos in firstPositions)
      if (secondPositions.contains(pos)) pos,
  };
}

int getShortestDistance(Iterable<Point<int>> points) {
  final distances = [
    for (final point in points) point.manhattanDistance(),
  ];

  return distances.reduce(min);
}

int getShortestCombinedLengthIntersection({
  required Iterable<Point<int>> intersections,
  required Iterable<Point<int>> firstWirePositions,
  required Iterable<Point<int>> secondWirePositions,
}) {
  final lengths = <int>[];

  for (final intersection in intersections) {
    final firstWireLength = firstWirePositions.takeWhile((p) => p != intersection).length + 1;
    final secondWireLength = secondWirePositions.takeWhile((p) => p != intersection).length + 1;

    lengths.add(firstWireLength + secondWireLength);
  }

  return lengths.reduce(min);
}

extension StringToIntExt on String {
  int toInt() => int.parse(this);
}

extension PointManhattanDistanceExt on Point<int> {
  int manhattanDistance() {
    return x.abs() + y.abs();
  }
}
