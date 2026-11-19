program TextfileStringParametersAndFunctionResultsAroundIo;
{ string temporaries survive calls that read and write files }
function ReadFirst(name: string): string;
var
  f: Text;
  s: string;
begin
  Assign(f, name);
  Reset(f);
  ReadLn(f, s);
  Close(f);
  ReadFirst := s;
end;
procedure Store(name, body: string);
var
  f: Text;
begin
  Assign(f, name);
  Rewrite(f);
  WriteLn(f, body);
  Close(f);
end;
begin
  Store('X.TMP', 'alpha' + ',' + 'beta');
  Store('Y.TMP', ReadFirst('X.TMP') + '!');
  WriteLn(ReadFirst('Y.TMP') + ReadFirst('X.TMP'));
  WriteLn(Length(ReadFirst('Y.TMP')));
end.
