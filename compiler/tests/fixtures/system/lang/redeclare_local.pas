program RedeclareLocal;
function Pick(Hi: Integer): Integer;
var Lo: Integer;
begin
  Lo := Hi + 1;
  Pick := Lo * 10
end;
procedure Outer;
  function UpCase(n: Integer): Integer;
  begin
    UpCase := n - 1
  end;
var ExitCode: Integer;
begin
  ExitCode := UpCase(10);
  WriteLn(ExitCode, ' ', Pick(4))
end;
begin
  Outer;
  WriteLn(Lo(513), ' ', Hi(513), ' ', ExitCode, ' ', UpCase('x'))
end.
