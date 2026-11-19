program assign_empty_output;
var
  f: Text;
begin
  Assign(f, '');
  Rewrite(f);
  WriteLn(f, 'to the console ', 42);
  Write(f, 'no newline');
  Close(f);
  WriteLn;
  WriteLn('after close');
end.
