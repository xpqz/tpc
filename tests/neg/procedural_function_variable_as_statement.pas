program NegTest;
{$F+}
type TProc = procedure; TFun = function(x: Integer): Integer; TGet = function: Integer;
var p: TProc; f: TFun; g: TGet; n: Integer;
procedure A;
begin
end;
function Id(x: Integer): Integer;
begin
  Id := x
end;
function One: Integer;
begin
  One := 1
end;
begin
  f := Id;
  f(1)
end.
