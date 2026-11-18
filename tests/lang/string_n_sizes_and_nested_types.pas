program StringSizes;
type S255 = String[255]; S1 = String[1]; S20 = String[20]; Name = String[30];
var a: S1; b: S20; c: Name;
begin
  WriteLn(SizeOf(a), ' ', SizeOf(b), ' ', SizeOf(c), ' ', SizeOf(String), ' ', SizeOf(S255))
end.
