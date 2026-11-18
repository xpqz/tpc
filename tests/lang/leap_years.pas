program LeapYears;
var y: Integer;
function Leap(y: Integer): Boolean;
begin
  Leap := (y mod 4 = 0) and ((y mod 100 <> 0) or (y mod 400 = 0))
end;
begin
  for y := 1896 to 1904 do WriteLn(y, ' ', Leap(y));
  WriteLn(1900, ' ', Leap(1900));
  WriteLn(2000, ' ', Leap(2000));
  WriteLn(2100, ' ', Leap(2100));
  WriteLn(1600, ' ', Leap(1600))
end.
