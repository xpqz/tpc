program ReadOfANumberFollowedByLettersIs106;
var
  f: Text;
  a, b: Integer;
begin
  Assign(f, 'LETTERS.TXT');
  Reset(f);
  Read(f, a);
  WriteLn(a);
  Read(f, b);
  WriteLn('not reached');
end.
