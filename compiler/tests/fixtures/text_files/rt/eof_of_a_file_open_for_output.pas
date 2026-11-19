program EofOfAFileOpenForOutput;
var
  f: Text;
begin
  Assign(f, 'OUT.TXT');
  Rewrite(f);
  WriteLn('before');
  if Eof(f) then WriteLn('eof');
  WriteLn('not reached');
end.
