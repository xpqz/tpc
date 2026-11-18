program VarParamNarrowStoreWraps;
{ arithmetic stored through a var parameter wraps at the parameter's width }
procedure AddB(var x: Byte; n: Integer); begin x := x + n end;
procedure AddS(var x: ShortInt; n: Integer); begin x := x + n end;
procedure AddI(var x: Integer; n: Integer); begin x := x + n end;
procedure AddW(var x: Word; n: LongInt); begin x := x + n end;
var b: Byte; s: ShortInt; i: Integer; w: Word;
begin
  b := 250; s := 120; i := 32000; w := 65000;
  AddB(b, 10); AddS(s, 10); AddI(i, 1000); AddW(w, 1000);
  WriteLn(b, ' ', s, ' ', i, ' ', w);
  AddB(b, -10); AddS(s, -20); AddI(i, -1000); AddW(w, -1000);
  WriteLn(b, ' ', s, ' ', i, ' ', w)
end.
