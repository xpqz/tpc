program StringVarParam;
procedure Up(var s: String);
var i: Integer;
begin
  for i := 1 to Length(s) do if (s[i] >= 'a') and (s[i] <= 'z') then s[i] := Chr(Ord(s[i]) - 32)
end;
procedure App(var s: String; c: Char);
begin
  s := s + c
end;
var a: String; b: String;
begin
  a := 'mixed Case 123';
  Up(a);
  WriteLn(a);
  App(a, '?'); App(a, '!');
  WriteLn(a, Length(a));
  b := '';
  App(b, 'x');
  WriteLn(b)
end.
