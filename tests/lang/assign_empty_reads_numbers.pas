program assign_empty_reads_numbers;
var
  f: Text;
  a, b: Integer;
begin
  Assign(f, '');
  Reset(f);
  ReadLn(f, a, b);
  WriteLn(a + b);
  Read(f, a);
  WriteLn(a * 2);
end.
