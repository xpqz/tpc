program RedeclareVarAndFunction;
var Length: Integer;
var ExitCode: LongInt;
function Lo(x: Integer): Integer;
begin
  Lo := 42
end;
function UpCase(n: Integer): Integer;
begin
  UpCase := n + 1
end;
begin
  Length := 5;
  ExitCode := 100000;
  WriteLn(Length, ' ', ExitCode, ' ', Lo(1), ' ', Hi(258), ' ', UpCase(9))
end.
