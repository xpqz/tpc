program Redeclare;
var i: Integer; s: String;
procedure Inc(var x: Integer; by: Integer);
begin
  x := x + by * 100
end;
procedure Dec(var x: Integer);
begin
  x := 0
end;
procedure Str(n: Integer);
begin
  WriteLn('mine ', n)
end;
procedure Val(n: Integer; m: Integer);
begin
  WriteLn('val ', n + m)
end;
function ParamCount: Integer;
begin
  ParamCount := 42
end;
function ParamStr(n: Integer): String;
begin
  ParamStr := 'arg'
end;
function Swap(n: Integer): Integer;
begin
  Swap := -n
end;
function Hi(n: Integer): Integer;
begin
  Hi := n * 2
end;
begin
  i := 1;
  Inc(i, 2);
  WriteLn(i);
  Dec(i);
  WriteLn(i);
  Str(7);
  Val(1, 2);
  WriteLn(ParamCount, ' ', ParamStr(3), ' ', Swap(5), ' ', Hi(8))
end.
