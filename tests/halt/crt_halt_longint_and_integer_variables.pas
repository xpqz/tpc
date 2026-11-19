program H15;
uses Crt;
var i: Integer; l: LongInt;
begin
  i := 41;
  l := 42;
  WriteLn(i, ' ', l);
  if i < l then Halt(l);
  Halt(i)
end.
