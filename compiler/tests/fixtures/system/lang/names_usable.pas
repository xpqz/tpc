program SystemNames;
var x: Integer; w: Word; c: Char;
function Twice(n: Integer): Integer;
begin
  Twice := Lo(n) * 2 + Hi(n)
end;
begin
  x := -2;
  w := 513;
  WriteLn(Lo(x), ' ', Hi(x), ' ', Lo(65535), ' ', Hi(258), ' ', Lo(w), ' ', Hi(w));
  WriteLn(Twice(258), ' ', Twice(-1), ' ', Lo(Hi(w) + 300));
  WriteLn(ExitCode);
  c := UpCase('a');
  WriteLn(c, UpCase('Z'), UpCase('3'), UpCase(Chr(122)), UpCase(Chr(96)), UpCase(Chr(123)));
  if UpCase('b') = 'B' then WriteLn('upper');
  if Lo(300) = 44 then WriteLn('lo')
end.
