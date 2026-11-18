program VarParamIntegerTypes;
{ var parameters of the integer types: store wraps at the width of the type }
procedure B(var x: Byte); begin x := x + 1 end;
procedure S(var x: ShortInt); begin x := x + 1 end;
procedure I(var x: Integer); begin x := x + 1 end;
procedure W(var x: Word); begin x := x + 1 end;
procedure L(var x: LongInt); begin x := x + 1 end;
var xb: Byte; xs: ShortInt; xi: Integer; xw: Word; xl: LongInt;
begin
  xb := 255; xs := 127; xi := 32767; xw := 65535; xl := 2147483647;
  B(xb); S(xs); I(xi); W(xw); L(xl);
  WriteLn(xb, ' ', xs, ' ', xi, ' ', xw, ' ', xl)
end.
