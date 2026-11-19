program FileBadIntegerInText;
var
  f: Text;
  a, b: Integer;
begin
  Assign(f, 'NUMS.TXT');
  Reset(f);
  ReadLn(f, a);
  WriteLn(a);
  ReadLn(f, b);
  WriteLn('not reached');
end.
