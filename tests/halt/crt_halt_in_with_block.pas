program H13;
uses Crt;
type Rec = record code: Integer end;
var r: Rec;
begin
  r.code := 12;
  with r do
  begin
    WriteLn('in with');
    Halt(code)
  end
end.
