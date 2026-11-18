program WriteWidthNarrowerThanValue;
var l: LongInt;
begin
  l := 1234567;
  WriteLn(l:1);
  WriteLn(l:3);
  WriteLn(l:6);
  WriteLn(l:7);
  WriteLn(l:8);
  WriteLn(65535:2);
  WriteLn('abcdef':3);
  WriteLn('abcdef':6);
  WriteLn('abcdef':7)
end.
