program StrTargets;
type Rec = record name: String[6]; n: Integer end;
var
  w: Word; sh: ShortInt; b: Byte; l: LongInt; i, k: Integer;
  s: String; t: String[1]; u: String[5];
  r: Rec; a: array[1..2] of String[4]; p: ^String;
procedure Fill(var out: String; v: LongInt; width: Integer);
begin
  Str(v:width, out)
end;
function Show(v: LongInt): String;
var tmp: String;
begin
  Str(v, tmp);
  Show := '<' + tmp + '>'
end;
begin
  w := 65535;
  Str(w, s);
  WriteLn(s, ' ', Length(s));
  sh := -128;
  Str(sh, s);
  WriteLn(s);
  b := 0;
  Str(b, s);
  WriteLn(s);
  l := -2147483647;
  Dec(l);
  Str(l, s);
  WriteLn(s, ' ', Length(s));
  Str(l:15, s);
  WriteLn('[', s, ']');
  Str(l:11, s);
  WriteLn('[', s, ']');
  Str(l:10, s);
  WriteLn('[', s, ']');
  Str(-5:-3, s);
  WriteLn('[', s, ']');
  Str(0:1, s);
  WriteLn('[', s, ']');
  Str(42:0, s);
  WriteLn('[', s, ']');
  Str(12345, t);
  WriteLn('[', t, ']');
  Str(-9, t);
  WriteLn('[', t, ']');
  Str(1234567, u);
  WriteLn('[', u, ']');
  Str(42:9, u);
  WriteLn('[', u, ']');
  Str(77, r.name);
  Str(r.n, r.name);
  r.n := 1234;
  Str(r.n, r.name);
  WriteLn(r.name);
  Str(123456789, r.name);
  WriteLn(r.name);
  Str(5, a[1]);
  Str(-123456, a[2]);
  WriteLn(a[1], '|', a[2]);
  New(p);
  Str(31337, p^);
  WriteLn(p^);
  Dispose(p);
  for k := 1 to 3 do
  begin
    Str(k * k, s);
    Write(s, ',')
  end;
  WriteLn;
  i := 12;
  Str(i, s);
  s := s + s;
  Str(Length(s), t);
  WriteLn(s, ' ', t);
  Str(i, s);
  Str(i * 100, s);
  WriteLn(s);
  Fill(s, 17, 6);
  WriteLn('[', s, ']');
  Fill(s, -17, 0);
  WriteLn('[', s, ']');
  WriteLn(Show(5), Show(-12), Show(0));
  Str(Ord('A'), s);
  WriteLn(s);
  Str(i:k, s);
  WriteLn('[', s, ']');
  Str(32767, s);
  WriteLn(s)
end.
