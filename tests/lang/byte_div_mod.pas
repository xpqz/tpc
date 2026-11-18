program ByteDivMod;
var a, b: Byte;
begin
  a := 255;
  b := 16;
  WriteLn(a div b, ' ', a mod b);
  a := 200;
  b := 7;
  WriteLn(a div b, ' ', a mod b);
  a := 3;
  b := 200;
  WriteLn(a div b, ' ', a mod b)
end.
