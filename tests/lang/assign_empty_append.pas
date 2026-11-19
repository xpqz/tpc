program assign_empty_append;
var
  f: Text;
begin
  Assign(f, '');
  Append(f);
  WriteLn(f, 'appended');
  Close(f);
end.
