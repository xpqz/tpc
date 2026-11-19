program NegTest;
{$F+}
type TShow = procedure(n: Integer);
var t: array[1..2] of TShow;
procedure Show(n: Integer);
begin
  WriteLn(n)
end;
begin
  t[1] := Show;
  t[1]
end.
