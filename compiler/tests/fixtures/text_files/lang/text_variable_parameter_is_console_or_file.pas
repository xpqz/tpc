program TextVariableParameterIsConsoleOrFile;
{ one routine writes to whatever file it is given, the screen included }
var
  f: Text;
  s: string;

procedure Show(var t: Text; n: Integer);
begin
  WriteLn(t, 'line ', n);
  Write(t, 'two ');
  WriteLn(t, 'pieces');
end;

procedure Echo(var t: Text);
var
  line: string;
begin
  while not Eof(t) do
  begin
    ReadLn(t, line);
    WriteLn('echo ', line);
  end;
end;

begin
  Show(Output, 1);
  Assign(f, 'PARAM.TXT');
  Rewrite(f);
  Show(f, 2);
  Close(f);
  Reset(f);
  Echo(f);
  Close(f);
end.
