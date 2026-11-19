program FileErrorAfterPartialOutputLine;
var
  f: Text;
begin
  Write('partial line');
  Assign(f, 'MISSING.TXT');
  Reset(f);
end.
