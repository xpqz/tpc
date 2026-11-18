program VarParamSwap;
{ classic swap over Integer, LongInt and Boolean }
procedure SwapI(var a, b: Integer);
var t: Integer;
begin t := a; a := b; b := t end;
procedure SwapL(var a, b: LongInt);
var t: LongInt;
begin t := a; a := b; b := t end;
procedure SwapB(var a, b: Boolean);
var t: Boolean;
begin t := a; a := b; b := t end;
var i, j: Integer; l, m: LongInt; p, q: Boolean;
begin
  i := 1; j := -2; l := 100000; m := -100000; p := True; q := False;
  SwapI(i, j); SwapL(l, m); SwapB(p, q);
  WriteLn(i, ' ', j, ' ', l, ' ', m, ' ', p, ' ', q);
  SwapI(i, i);
  WriteLn(i)
end.
