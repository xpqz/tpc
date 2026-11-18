program RecursionDepth1000;
function Sum(n: Integer): LongInt;
begin
  if n = 0 then Sum := 0
  else Sum := n + Sum(n - 1)
end;
function Depth(n: Integer): Integer;
begin
  if n = 0 then Depth := 0
  else Depth := 1 + Depth(n - 1)
end;
procedure Down(n: Integer);
begin
  if n > 0 then Down(n - 1)
  else WriteLn('bottom')
end;
begin
  WriteLn(Sum(1000));
  WriteLn(Depth(1000));
  Down(1000);
  WriteLn(Sum(1))
end.
