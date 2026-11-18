program DivModLongInt;
var a, b: LongInt;
begin
  a := 2147483647;
  b := 1000;
  WriteLn(a div b, ' ', a mod b);
  a := -2147483647;
  WriteLn(a div b, ' ', a mod b);
  b := -7;
  WriteLn(a div b, ' ', a mod b);
  a := 1000000;
  b := 3;
  WriteLn(a div b, ' ', a mod b)
end.
