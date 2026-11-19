program FileBadRealInText;
var
  f: Text;
  r: Real;
begin
  Assign(f, 'REALS.TXT');
  Reset(f);
  Read(f, r);
  WriteLn(r:0:1);
  Read(f, r);
  WriteLn('not reached');
end.
