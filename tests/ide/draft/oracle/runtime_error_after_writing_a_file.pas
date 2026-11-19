program D;
var
  T: Text;
  Zero: Integer;
begin
  Assign(T, 'OUT.TXT');
  Rewrite(T);
  WriteLn(T, 'written by the program');
  Close(T);
  Zero := 0;
  WriteLn(1 div Zero);
end.
