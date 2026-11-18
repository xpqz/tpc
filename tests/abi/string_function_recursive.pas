program StringRecursive;
function Bin(n: Integer): String;
begin
  if n < 2 then Bin := Chr(48 + n)
  else Bin := Bin(n div 2) + Chr(48 + n mod 2)
end;
function Rev(s: String): String;
begin
  if Length(s) <= 1 then Rev := s else Rev := Rev(Copy(s, 2, 255)) + s[1]
end;
begin
  WriteLn(Bin(0), ' ', Bin(5), ' ', Bin(255), ' ', Bin(1000));
  WriteLn(Rev('recursion'), ' ', Rev(''), '|', Rev('x'))
end.
