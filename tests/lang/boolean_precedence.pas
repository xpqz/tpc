program BooleanPrecedence;
var a, b, c: Boolean;
begin
  a := True;
  b := False;
  c := True;
  WriteLn(a or b and c);
  WriteLn((a or b) and c);
  WriteLn(not a or b);
  WriteLn(not (a or b));
  WriteLn(a and not b);
  WriteLn(a xor b xor c);
  WriteLn((1 < 2) and (2 < 3));
  WriteLn((1 < 2) = (3 < 4))
end.
