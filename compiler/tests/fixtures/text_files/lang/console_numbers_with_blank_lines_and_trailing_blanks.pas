program ConsoleNumbersWithBlankLinesAndTrailingBlanks;
{ Read of a number skips blanks and line ends; at the end of the input it gives 0 }
var
  a, b, c: Integer;
begin
  Read(a);
  Read(b);
  Read(c);
  WriteLn(a, ' ', b, ' ', c, ' ', Eof, ' ', Eoln);
  Read(a);
  WriteLn(a, ' ', Eof);
end.
