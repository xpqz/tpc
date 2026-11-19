program TypedconstScalarsAreMutable;
type
  TColor = (Red, Green, Blue);
const
  i: Integer = 1;
  c: Char = 'a';
  flag: Boolean = False;
  col: TColor = Red;
  r: Real = 1.5;
  l: LongInt = 100000;
begin
  i := i + 41;
  c := Succ(c);
  flag := not flag;
  col := Succ(col);
  r := r * 2;
  l := l * 3;
  WriteLn(i, ' ', c, ' ', flag, ' ', Ord(col), ' ', r:0:1, ' ', l);
  i := i + 1;
  Inc(i);
  WriteLn(i);
end.
