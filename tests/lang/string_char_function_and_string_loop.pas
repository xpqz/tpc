program StringCharFunc;
function Roman(n: Integer): String;
var s: String;
begin
  s := '';
  while n >= 1000 do begin s := s + 'M'; n := n - 1000 end;
  while n >= 100 do begin s := s + 'C'; n := n - 100 end;
  while n >= 10 do begin s := s + 'X'; n := n - 10 end;
  while n >= 1 do begin s := s + 'I'; n := n - 1 end;
  Roman := s
end;
begin
  WriteLn(Roman(1), Roman(4), ' ', Roman(23), ' ', Roman(1999))
end.
