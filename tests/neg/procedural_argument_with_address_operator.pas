program NegTest;
{$F+}
type TFunc = function(x: Integer): Integer;
function Id(x: Integer): Integer;
begin
  Id := x
end;
procedure Use(f: TFunc);
begin
end;
begin
  Use(@Id)
end.
