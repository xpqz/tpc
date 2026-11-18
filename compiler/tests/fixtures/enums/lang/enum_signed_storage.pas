program EnumSignedStorage;
type Color = (Red, Green, Blue);
var c: Color;
procedure Show(x: Color);
begin
  WriteLn(Ord(x), ' ', x < Red, ' ', x > Blue)
end;
procedure Dec(var x: Color);
begin
  x := Pred(x)
end;
begin
  c := Red;
  Dec(c);
  Show(c);
  WriteLn(Ord(c));
  c := Pred(c);
  Show(c)
end.
