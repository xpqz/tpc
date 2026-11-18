program Caesar;
function Shift(s: String; k: Integer): String;
var i: Integer; c: Char;
begin
  for i := 1 to Length(s) do begin
    c := s[i];
    if (c >= 'a') and (c <= 'z') then s[i] := Chr((Ord(c) - 97 + k) mod 26 + 97)
    else if (c >= 'A') and (c <= 'Z') then s[i] := Chr((Ord(c) - 65 + k) mod 26 + 65)
  end;
  Shift := s
end;
var m: String;
begin
  m := 'Hello, World';
  WriteLn(Shift(m, 3));
  WriteLn(Shift(Shift(m, 3), 23));
  WriteLn(m)
end.
