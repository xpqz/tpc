program VarParamAllScalarTypes;
{ a var parameter of each scalar type reads and writes the caller's variable at its width }
procedure SetB(var x: Byte); begin x := x + 1 end;
procedure SetS(var x: ShortInt); begin x := x + 1 end;
procedure SetI(var x: Integer); begin x := x + 1 end;
procedure SetW(var x: Word); begin x := x + 1 end;
procedure SetL(var x: LongInt); begin x := x + 1 end;
procedure SetT(var x: Boolean); begin x := not x end;
procedure SetC(var x: Char); begin x := Chr(Ord(x) + 1) end;
var b: Byte; s: ShortInt; i: Integer; w: Word; l: LongInt; t: Boolean; c: Char;
begin
  b := 255; s := 127; i := 32767; w := 65535; l := 2147483647; t := True; c := 'y';
  SetB(b); SetS(s); SetI(i); SetW(w); SetL(l); SetT(t); SetC(c);
  WriteLn(b, ' ', s, ' ', i, ' ', w, ' ', l, ' ', t, ' ', c);
  b := 1; s := -1; i := -1; w := 0; l := -1; t := False; c := #64;
  SetB(b); SetS(s); SetI(i); SetW(w); SetL(l); SetT(t); SetC(c);
  WriteLn(b, ' ', s, ' ', i, ' ', w, ' ', l, ' ', t, ' ', c)
end.
