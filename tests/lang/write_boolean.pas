program WriteBoolean;
var a, b: Boolean;
begin
  WriteLn(True);
  WriteLn(False);
  a := 3 < 4;
  b := not a;
  WriteLn(a, ' ', b);
  WriteLn(1 = 2);
  WriteLn(2 > 1);
  Write(a and b, ' ');
  Write(a or b, ' ');
  WriteLn(a xor b)
end.
