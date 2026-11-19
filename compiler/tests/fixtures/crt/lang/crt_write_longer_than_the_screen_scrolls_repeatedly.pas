program LongWrite;
uses Crt;
var s: string;
    i: Integer;
begin
  ClrScr;
  GotoXY(1, 25);
  s := '';
  for i := 1 to 200 do
    s := s + Chr(Ord('a') + i mod 26);
  Write(s);
  GotoXY(1, 1);
  Write('top');
end.
