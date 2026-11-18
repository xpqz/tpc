program EnumShadowing;
type Color = (Red, Green, Blue);
var c: Color;
procedure P;
var Red: Integer;
begin
  Red := 40;
  WriteLn(Red + 2)
end;
procedure Q(Green: Integer);
begin
  WriteLn(Green)
end;
begin
  P; Q(7);
  c := Red;
  WriteLn(Ord(c))
end.
