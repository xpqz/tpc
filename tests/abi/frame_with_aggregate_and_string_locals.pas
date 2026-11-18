program FrameAggregates;
procedure P(n: Integer);
var s: String; a: array[1..10] of Integer; t: String[7]; i: Integer;
begin
  s := 'level';
  t := 'abcdefghij';
  for i := 1 to 10 do a[i] := n * i;
  if n > 0 then P(n - 1);
  WriteLn(n, ' ', s, ' ', t, ' ', a[10])
end;
begin
  P(3)
end.
