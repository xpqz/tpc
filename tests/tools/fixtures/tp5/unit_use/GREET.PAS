unit Greet;
interface
const Name = 'unit';
procedure Hello(s: string);
implementation
procedure Hello(s: string);
begin
  WriteLn('hello ', s)
end;
end.
