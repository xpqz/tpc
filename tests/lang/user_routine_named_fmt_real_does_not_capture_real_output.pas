program UserFmtReal;
var x: Real;

function Fmt_Real(v: Real; width, decimals, buffer: LongInt): LongInt;
begin
  Fmt_Real := 0
end;

begin
  x := 1.5;
  WriteLn(x:0:2);
  WriteLn(Fmt_Real(x, 0, 2, 0))
end.
