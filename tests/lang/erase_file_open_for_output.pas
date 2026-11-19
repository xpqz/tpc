program EraseFileOpenForOutput;
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

  Assign(f, 'OUT.TXT');
  Rewrite(f);
  WriteLn(f, 'first');
  Erase(f);
  Report('erase open file');
  WriteLn(f, 'second');
  Report('write after erase');
  Close(f);
  Report('close');
  Show('OUT.TXT');
end.
