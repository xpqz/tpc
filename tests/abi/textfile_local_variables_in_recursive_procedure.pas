program TextfileLocalVariablesInRecursiveProcedure;
{ each activation owns a file variable and a distinct file }
procedure Level(n: Integer);
var
  f: Text;
  name: string[8];
  s: string;
begin
  Str(n, name);
  name := 'LV' + name + '.TMP';
  Assign(f, name);
  Rewrite(f);
  WriteLn(f, 'level ', n);
  if n < 3 then Level(n + 1);
  Close(f);
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
end;
begin
  Level(1);
end.
