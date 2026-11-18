program FrameValueParamCopiedIn;
{ a value parameter that is passed on as a var argument lives in the frame: the incoming
  value is copied in at entry, and changes through the var parameter are seen by the body }
procedure Bump(var x: Integer); begin x := x + 1 end;
procedure BumpB(var x: Byte); begin x := x + 1 end;
procedure WrapW(var x: Word); begin x := x + 1 end;
procedure Callee(i: Integer; b: Byte; w: Word; l: LongInt);
var snapshot: Integer;
begin
  snapshot := i;
  Bump(i); BumpB(b); WrapW(w);
  WriteLn(i, ' ', b, ' ', w, ' ', l, ' ', snapshot);
  l := l + i;
  WriteLn(l)
end;
var i: Integer; b: Byte; w: Word;
begin
  i := 32767; b := 255; w := 65535;
  Callee(i, b, w, 7);
  WriteLn(i, ' ', b, ' ', w);
  Callee(-1, 0, 0, -7)
end.
