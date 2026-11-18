program RangeTwo;
{$R+}
var m: array[1..3, 1..3] of Integer; i, j: Integer;
begin
  WriteLn('before');
  i := 2;
  j := 4;
  m[i, j] := 1;
  WriteLn(m[1, 1]);
  WriteLn('not reached')
end.
