program Crash;
uses Crt;
var
  Zero: Integer;
begin
  ClrScr;
  Zero := 0;
  Write('before');
  Write(1 div Zero);
end.
