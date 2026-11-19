unit Loader;
interface
function FirstLine(name: string): string;
implementation
function FirstLine(name: string): string;
var
  f: Text;
  s: string;
begin
  Assign(f, name);
  Reset(f);
  ReadLn(f, s);
  Close(f);
  FirstLine := s;
end;
end.
