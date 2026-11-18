program StringWriteWidthExpr;
var s: String; i: Integer;
begin
  s := 'abcd';
  for i := 0 to 7 do WriteLn(s:i, '|');
  WriteLn(s:2 + 4, '|', s:Length(s) * 2, '|')
end.
