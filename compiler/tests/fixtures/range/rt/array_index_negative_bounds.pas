program NegBounds;
{$R+}
var m: array[-3..3] of Integer; i: Integer;
begin
  i := -3;
  m[i] := 1;
  i := 3;
  m[i] := 2;
  WriteLn(m[-3] + m[3]);
  i := -4;
  m[i] := 3;
  WriteLn('not reached')
end.
