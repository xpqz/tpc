program NegTest;
{$F+}
type TFunc = function: Integer;
var f: TFunc; t: Boolean;
function One: Integer;
begin
  One := 1
end;
begin
  f := One;
  t := f
end.
