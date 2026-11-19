program random_sequences;
{ TP5's generator: RandSeed := RandSeed * 134775813 + 1; Random(n) is the high word of
  the new seed modulo n. }
var
  seed, i: Integer;
  n: Word;
  l: LongInt;

procedure Run(start: LongInt; range: Word; count: Integer);
var k: Integer;
begin
  RandSeed := start;
  write(start, ' ', range, ':');
  for k := 1 to count do
    write(' ', Random(range));
  writeln(' | ', RandSeed)
end;

begin
  { The seed is zero at the start of every program. }
  writeln(RandSeed);
  writeln(Random(1000), ' ', RandSeed);
  Run(5, 100, 8);
  Run(1, 1000, 6);
  Run(12345, 6, 10);
  Run(-1, 65535, 4);
  Run(2147483647, 7, 4);
  Run(-2147483647, 256, 4);
  Run(40000, 2, 12);
  Run(0, 1, 5);
  Run(0, 0, 3);
  Run(77, 65535, 3);
  { A Word and an Integer variable, a LongInt variable and a constant expression. }
  RandSeed := 9;
  n := 50;
  i := 20;
  l := 30;
  writeln(Random(n), ' ', Random(i), ' ', Random(l), ' ', Random(2 * 10 + 5));
  { The result is a Word, so it can be assigned to Word and LongInt variables. }
  RandSeed := 9;
  n := Random(40000);
  l := Random(40000);
  writeln(n, ' ', l);
  { Setting the seed again restarts the sequence. }
  RandSeed := 5;
  writeln(Random(100));
  RandSeed := 5;
  writeln(Random(100), ' ', Random(100) <> Random(100));
end.
