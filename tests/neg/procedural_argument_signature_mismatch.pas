program NegTest;
{$F+}
type TFunc = function(x: Integer): Integer;
function Two(a, b: Integer): Integer;
begin
  Two := a
end;
procedure Use(f: TFunc);
begin
end;
begin
  Use(Two)
end.
