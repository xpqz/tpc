program VarParamCharBoolean;
{ var parameters of Char and Boolean }
procedure Flip(var b: Boolean); begin b := not b end;
procedure Up(var c: Char); begin c := Chr(Ord(c) - 32) end;
var t: Boolean; c: Char;
begin
  t := False; c := 'q';
  Flip(t); Up(c);
  WriteLn(t, ' ', c);
  Flip(t);
  WriteLn(t)
end.
