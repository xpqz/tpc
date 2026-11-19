program swag_random_seeded;
begin
  Randomize;
  writeln(Random(1) );
  writeln(Random(10) < 10);
  RandSeed := 5;
  writeln(Random(100));
end.
