unit Hooks;
{$F+}
interface
type
  TNotify = procedure(s: string);
var
  OnEvent: TNotify;
  Fired: Integer;
procedure Fire(s: string);
procedure Ignore(s: string);
implementation
procedure Fire(s: string);
begin
  Fired := Fired + 1;
  OnEvent(s);
end;
procedure Ignore(s: string);
begin
end;
procedure Default(s: string);
begin
  WriteLn('default: ', s);
end;
begin
  OnEvent := Default;
  Fired := 0;
end.
