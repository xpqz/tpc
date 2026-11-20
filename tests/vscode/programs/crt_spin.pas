uses Crt;
var n: Integer;
begin
  ClrScr;
  n := 0;
  while True do
  begin
    GotoXY(1, 1);
    Write(n);
    n := n + 1;
  end;
end.
