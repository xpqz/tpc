program CharIndex;
{$R+}
var a: array['a'..'e'] of Integer; c: Char;
begin
  c := 'e';
  a[c] := 1;
  WriteLn(a[c]);
  c := 'f';
  WriteLn(a[c]);
  WriteLn('not reached')
end.
