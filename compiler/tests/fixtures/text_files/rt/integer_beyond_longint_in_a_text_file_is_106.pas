program IntegerBeyondLongintInATextFileIs106;
var
  f: Text;
  n: LongInt;
begin
  Assign(f, 'BIG.TXT');
  Reset(f);
  Read(f, n);
  WriteLn(n);
  Read(f, n);
  WriteLn('not reached');
end.
