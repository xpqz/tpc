program RedeclareConstAndType;
const Hi = 5; ExitCode = 'none';
type Lo = Byte; UpCase = Integer;
var a: Lo; b: UpCase;
begin
  a := 200;
  b := Hi * 1000;
  WriteLn(a, ' ', b, ' ', Hi, ' ', ExitCode)
end.
