program DosFSplitNeedsDirStrVariable;
uses Dos;
var
  d: string;
  n: NameStr;
  e: ExtStr;
begin
  FSplit('A.B', d, n, e);
end.
