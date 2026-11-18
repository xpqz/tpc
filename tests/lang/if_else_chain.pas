program IfElseChain;
var n: Integer;
procedure Classify(n: Integer);
begin
  if n < 0 then
    WriteLn(n, ' negative')
  else if n = 0 then
    WriteLn(n, ' zero')
  else if n < 10 then
    WriteLn(n, ' small')
  else
    WriteLn(n, ' large')
end;
begin
  Classify(-3);
  Classify(0);
  Classify(5);
  Classify(10);
  Classify(1000)
end.
