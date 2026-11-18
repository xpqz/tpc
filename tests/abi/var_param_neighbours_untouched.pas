program VarParamNeighboursUntouched;
{ a store through a narrow var parameter must not disturb adjacent variables }
var a, b, c: Byte; x, y, z: Integer; p, q: ShortInt;
procedure Fill(var v: Byte; n: Byte); begin v := n end;
procedure FillI(var v: Integer; n: Integer); begin v := n end;
procedure FillS(var v: ShortInt; n: ShortInt); begin v := n end;
begin
  a := 1; b := 2; c := 3; x := 1000; y := 2000; z := 3000; p := 4; q := 5;
  Fill(b, 255); FillI(y, -1); FillS(p, -128);
  WriteLn(a, ' ', b, ' ', c);
  WriteLn(x, ' ', y, ' ', z);
  WriteLn(p, ' ', q);
  Fill(b, 0); FillI(y, 0); FillS(p, 0);
  WriteLn(a, ' ', b, ' ', c, ' ', x, ' ', y, ' ', z, ' ', p, ' ', q)
end.
