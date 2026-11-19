program switches_and_m_keep_r_and_i_working;
{$A+,I-,R+,S-}
{$M 16384,0,655360}
var
  a: array[1..3] of integer;
  f: text;
  i: integer;
begin
  i := 3;
  a[i] := 7;
  writeln(a[i]);
  assign(f, 'NO_SUCH_FILE.TXT');
  reset(f);
  writeln(ioresult <> 0);
end.
