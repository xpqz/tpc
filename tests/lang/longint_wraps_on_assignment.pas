program LongIntWrapsOnAssignment;
var l: LongInt;
begin
  l := 2147483647;
  l := l + 1;
  WriteLn(l);
  l := -2147483647 - 1;
  l := l - 1;
  WriteLn(l);
  l := 65536;
  l := l * 65536;
  WriteLn(l);
  l := 2000000000;
  l := l + l;
  WriteLn(l)
end.
