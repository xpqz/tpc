program UnitCrossUnitProceduralTypes;
{$F+}
{ a procedural type declared in one unit is implemented by routines of another unit }
uses Callback, Ops;
var
  f: TBinary;
  i: Integer;
  total: Integer;
procedure Collect(n: Integer);
begin
  total := total + n;
end;
begin
  f := Add;
  WriteLn(f(3, 4));
  f := Mul;
  WriteLn(f(3, 4));
  WriteLn(Fold(Add, 1, 5));
  WriteLn(Fold(Mul, 1, 5));
  total := 0;
  EachUpTo(5, Collect);
  WriteLn(total);
  for i := 0 to 1 do
    WriteLn(Table[i](10, 3));
  Pick(True, f);
  WriteLn(f(8, 2));
  Pick(False, f);
  WriteLn(f(8, 2));
end.
