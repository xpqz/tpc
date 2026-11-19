program ResultSize;
uses IdeHost;
var
  R: CompileResult;
begin
  WriteLn(SizeOf(CompileResult));
  WriteLn(SizeOf(R.Code), ' ', SizeOf(R.Line), ' ', SizeOf(R.Col), ' ', SizeOf(R.FileName));
  R.Code := -2;
  R.Line := 70000;
  R.Col := 100000;
  R.FileName := 'ABCDEFGHIJKLMNOP';
  WriteLn(R.Code, ' ', R.Line, ' ', R.Col, ' ', R.FileName);
end.
