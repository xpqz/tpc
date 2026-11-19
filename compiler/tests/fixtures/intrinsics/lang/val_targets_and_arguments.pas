program ValTargets;
type Rec = record n: Integer; r: Real end;
var
  i, e: Integer; l: LongInt; w: Word; b: Byte; sh: ShortInt; r: Real;
  s: String; t: String[4]; c: Char; rc: Rec; a: array[1..3] of Integer; p: ^LongInt;
  codes: array[1..2] of Integer;
procedure Parse(const_text: String; var into: Integer; var code: Integer);
begin
  Val(const_text, into, code)
end;
begin
  Val('200', b, e);
  WriteLn(b, ' ', e);
  Val('-100', sh, e);
  WriteLn(sh, ' ', e);
  Val('-128', sh, e);
  WriteLn(sh, ' ', e);
  Val('40000', w, e);
  WriteLn(w, ' ', e);
  Val('70000', i, e);
  WriteLn(i, ' ', e);
  Val('70000', l, e);
  WriteLn(l, ' ', e);
  Val('-70000', w, e);
  WriteLn(w, ' ', e);
  Val('300', b, e);
  WriteLn(b, ' ', e);
  Val('$FFFF', i, e);
  WriteLn(i, ' ', e);
  Val('$FFFF', w, e);
  WriteLn(w, ' ', e);
  Val('$1FF', b, e);
  WriteLn(b, ' ', e);
  t := '1234';
  Val(t, i, e);
  WriteLn(i, ' ', e);
  Val(t + '5', i, e);
  WriteLn(i, ' ', e);
  c := '8';
  Val(c, i, e);
  WriteLn(i, ' ', e);
  Val(Copy('ab12cd', 3, 2), i, e);
  WriteLn(i, ' ', e);
  Val('77', rc.n, e);
  Val('2.5', rc.r, codes[1]);
  WriteLn(rc.n, ' ', rc.r:0:1, ' ', e, ' ', codes[1]);
  Val('12', a[2], codes[2]);
  Val('1x', a[3], codes[1]);
  WriteLn(a[2], ' ', a[3], ' ', codes[2], ' ', codes[1]);
  New(p);
  Val('-123456', p^, e);
  WriteLn(p^, ' ', e);
  Dispose(p);
  Parse('314', i, e);
  WriteLn(i, ' ', e);
  Parse('3 14', i, e);
  WriteLn(i, ' ', e)
end.
