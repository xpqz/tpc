program FrameDeepStrings;
function Count(n: Integer): Integer;
var s: String[20];
begin
  s := 'x';
  if n = 0 then Count := Length(s)
  else Count := Count(n - 1) + Length(s)
end;
begin
  WriteLn(Count(300))
end.
