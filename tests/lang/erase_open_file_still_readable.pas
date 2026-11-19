program EraseOpenFileStillReadable;
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

  Assign(f, 'DATA.TXT');
  Reset(f);
  Erase(f);
  Report('erase open file');
  ReadLn(f, line);
  Report('read');
  WriteLn(line);
  Close(f);
  Report('close');
  Show('DATA.TXT');
end.
