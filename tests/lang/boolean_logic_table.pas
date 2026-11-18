program BooleanLogicTable;
var a, b: Boolean;
procedure Row;
begin
  WriteLn(a:6, b:6, (a and b):6, (a or b):6, (a xor b):6, (not a):6)
end;
begin
  a := False; b := False; Row;
  a := False; b := True; Row;
  a := True; b := False; Row;
  a := True; b := True; Row
end.
