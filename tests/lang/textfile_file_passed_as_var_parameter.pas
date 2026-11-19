program TextfileFilePassedAsVarParameter;
procedure Emit(var f: Text; n: Integer);
var
  i: Integer;
begin
  for i := 1 to n do
    WriteLn(f, 'row ', i);
end;
function CountLines(var f: Text): Integer;
var
  n: Integer;
begin
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f);
    n := n + 1;
  end;
  CountLines := n;
end;
var
  f: Text;
begin
  Assign(f, 'ROWS.TXT');
  Rewrite(f);
  Emit(f, 5);
  Close(f);
  Reset(f);
  WriteLn(CountLines(f));
  Close(f);
end.
