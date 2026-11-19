program RenameFileOpenForOutput;
{$I-}
var
  f, g: Text;
  line: String;

procedure Report(what: String);
var code: Integer;
begin
  code := IOResult;
  WriteLn(what, ': ', code);
end;

{ Tells whether name opens for reading and shows its first line. }
procedure Show(name: String);
begin
  Assign(g, name);
  Reset(g);
  if IOResult = 0 then
  begin
    ReadLn(g, line);
    WriteLn(name, ' holds ', line);
    Close(g);
  end
  else
    WriteLn(name, ' is missing');
end;

procedure Make(name, text: String);
begin
  Assign(g, name);
  Rewrite(g);
  WriteLn(g, text);
  Close(g);
  Report('make ' + name);
end;

begin

  Assign(f, 'A.TXT');
  Rewrite(f);
  WriteLn(f, 'one');
  Rename(f, 'B.TXT');
  Report('rename open file');
  WriteLn(f, 'two');
  Close(f);
  Report('close');
  Show('A.TXT');
  Assign(g, 'B.TXT');
  Reset(g);
  while not Eof(g) do
  begin
    ReadLn(g, line);
    WriteLn('B.TXT line ', line);
  end;
  Close(g);
end.
