program IncludeNestedFiles;
{ an include file includes another one }
{$I OUTER.INC}
begin
  WriteLn(A + B + C);
  WriteLn(FromInner);
end.
