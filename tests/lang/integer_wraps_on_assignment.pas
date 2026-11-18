program IntegerWrapsOnAssignment;
var i: Integer;
begin
  i := 32767;
  i := i + 1;
  WriteLn(i);
  i := -32768;
  i := i - 1;
  WriteLn(i);
  i := 20000;
  i := i + i;
  WriteLn(i);
  i := 256;
  i := i * 256;
  WriteLn(i);
  i := 181;
  i := i * i;
  WriteLn(i)
end.
