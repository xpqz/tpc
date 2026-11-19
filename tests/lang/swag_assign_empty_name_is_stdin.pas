program swag_assign_empty_name_is_stdin;
var
  f: Text;
  s: string;
begin
  Assign(f, '');
  Reset(f);
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Close(f);
end.
