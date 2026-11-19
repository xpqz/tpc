program ConsoleMixedNumberThenTextOnOneLine;
{ after Read of a number the rest of the line, blank included, is still there }
var
  n: Integer;
  s: string;
begin
  Read(n);
  ReadLn(s);
  WriteLn(n, '[', s, ']');
  ReadLn(n, s);
  WriteLn(n, '[', s, ']');
end.
