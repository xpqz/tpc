program NumToText;
function Dec(n: LongInt): String;
var s: String; neg: Boolean;
begin
  if n = 0 then begin Dec := '0'; Exit end;
  neg := n < 0;
  if neg then n := -n;
  s := '';
  while n > 0 do begin s := Chr(48 + n mod 10) + s; n := n div 10 end;
  if neg then s := '-' + s;
  Dec := s
end;
begin
  WriteLn(Dec(0), ' ', Dec(7), ' ', Dec(-42), ' ', Dec(2147483647), ' ', Dec(100000));
  WriteLn(Length(Dec(1234567)))
end.
