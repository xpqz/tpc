program random_real;
{ Random without an argument is the new seed divided by 2^32, a Real in [0, 1). Scaling by a
  power of two is exact, so these values print the same in 48 and 64 bits. }
var
  r: Real;
  i, below: Integer;
  ok: Boolean;
begin
  RandSeed := 1;
  writeln(Random * 4294967296.0 : 0 : 0, ' ', RandSeed);
  RandSeed := 5;
  for i := 1 to 4 do
    write(Random * 4294967296.0 : 0 : 0, ' ');
  writeln;
  RandSeed := 123456;
  for i := 1 to 4 do
    write(Trunc(Random * 65536.0), ' ');
  writeln;
  RandSeed := 0;
  r := Random;
  writeln(r * 4294967296.0 : 0 : 0, ' ', r > 0.0, ' ', r < 0.0001);
  RandSeed := -1;
  r := Random;
  writeln(r * 4294967296.0 : 0 : 0, ' ', r < 1.0, ' ', r > 0.9);
  { A thousand values all lie in [0, 1) and about half are below 0.5. }
  RandSeed := 42;
  ok := True;
  below := 0;
  for i := 1 to 1000 do
  begin
    r := Random;
    if (r < 0.0) or (r >= 1.0) then ok := False;
    if r < 0.5 then below := below + 1
  end;
  writeln(ok, ' ', below);
  { Random in an expression, and the real and word forms share one seed. }
  RandSeed := 8;
  r := Random * 2.0 + 10.0;
  writeln(Trunc(r * 1024.0));
  writeln(Random(100), ' ', Trunc(Random * 100.0), ' ', RandSeed);
end.
