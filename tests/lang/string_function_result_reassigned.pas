program StringFunctionPartial;
function F(n: Integer): String;
var s: String;
begin
  F := 'a';
  s := 'bcd';
  if n > 0 then F := s;
  if n > 1 then F := s + 'e'
end;
begin
  WriteLn(F(0), ' ', F(1), ' ', F(2))
end.
