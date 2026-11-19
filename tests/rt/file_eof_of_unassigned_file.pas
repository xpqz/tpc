program FileEofOfUnassignedFile;
var
  f: Text;
begin
  WriteLn('before');
  if Eof(f) then WriteLn('eof');
  WriteLn('not reached');
end.
