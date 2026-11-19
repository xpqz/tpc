program FileReadFromFileOpenForOutput;
var
  f: Text;
  s: string;
begin
  Assign(f, 'OUT.TMP');
  Rewrite(f);
  WriteLn(f, 'x');
  WriteLn('writing');
  ReadLn(f, s);
  WriteLn('not reached');
end.
