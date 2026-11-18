program FrameMixedSizesLayout;
{ locals of every size passed as var arguments, interleaved with locals that are not:
  stores of each width must not touch the neighbours }
procedure Set8(var x: Byte; v: Byte); begin x := v end;
procedure SetS(var x: ShortInt; v: ShortInt); begin x := v end;
procedure Set16(var x: Integer; v: Integer); begin x := v end;
procedure SetW(var x: Word; v: Word); begin x := v end;
procedure Set32(var x: LongInt; v: LongInt); begin x := v end;
procedure SetB(var x: Boolean; v: Boolean); begin x := v end;
procedure SetC(var x: Char; v: Char); begin x := v end;
procedure Layout;
var a: Byte; plain1: Integer; b: ShortInt; c: Integer; d: Word; plain2: Byte;
    e: LongInt; f: Boolean; g: Char; h: LongInt;
begin
  plain1 := 1; plain2 := 2;
  a := 0; b := 0; c := 0; d := 0; e := 0; f := False; g := 'a'; h := 0;
  SetS(b, -1); Set8(a, 255);
  Set16(c, -2); SetW(d, 65534);
  Set32(e, -3); SetB(f, True); SetC(g, 'z'); Set32(h, 123456789);
  WriteLn(a, ' ', b, ' ', c, ' ', d, ' ', e, ' ', f, ' ', g, ' ', h);
  Set8(a, 0); SetS(b, 0);
  WriteLn(a, ' ', b, ' ', c, ' ', d, ' ', e, ' ', f, ' ', g, ' ', h);
  Set32(e, 0); SetW(d, 0);
  WriteLn(a, ' ', b, ' ', c, ' ', d, ' ', e, ' ', f, ' ', g, ' ', h, ' ', plain1, ' ', plain2)
end;
begin
  Layout;
  Layout
end.
