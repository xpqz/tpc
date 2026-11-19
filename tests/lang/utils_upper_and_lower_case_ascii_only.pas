program UtilsCase;
uses Utils;
var
  All, Changed: String;
  I: Integer;
begin
  WriteLn('[', UpperCase(''), '] [', LowerCase(''), ']');
  WriteLn(UpperCase('Hello, World 123'), ' ', LowerCase('Hello, World 123'));
  WriteLn(UpperCase('abcxyz'), ' ', LowerCase('ABCXYZ'));
  WriteLn(UpperCase('already UPPER'), ' ', LowerCase('already lower'));
  WriteLn(UpperCase('@[`{'), ' ', LowerCase('@[`{'));
  WriteLn(UpperCase('09 _-~'), ' ', LowerCase('09 _-~'));
  All := '';
  for I := 128 to 255 do
    All := All + Chr(I);
  WriteLn(UpperCase(All) = All, ' ', LowerCase(All) = All);
  All := '';
  for I := 0 to 127 do
    All := All + Chr(I);
  Changed := UpperCase(All);
  for I := 0 to 127 do
    if Changed[I + 1] <> All[I + 1] then
      Write(All[I + 1]);
  WriteLn;
  Changed := LowerCase(All);
  for I := 0 to 127 do
    if Changed[I + 1] <> All[I + 1] then
      Write(All[I + 1]);
  WriteLn;
  All := 'MiXeD';
  WriteLn(UpperCase(All), LowerCase(All), All);
end.
