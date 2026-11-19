program FlushOfClosedFile;
var f: Text;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  Rewrite(f);
  Close(f);
  Flush(f);
  WriteLn('not reached');
end.
