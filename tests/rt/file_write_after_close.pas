program FileWriteAfterClose;
var
  f: Text;
begin
  Assign(f, 'OUT.TMP');
  Rewrite(f);
  WriteLn(f, 'written');
  Close(f);
  WriteLn('closed');
  WriteLn(f, 'too late');
  WriteLn('not reached');
end.
