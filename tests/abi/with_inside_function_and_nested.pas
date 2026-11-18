program WithFunctionNested;
type RT = record n: Integer; s: String[4] end;
var r: RT;
function Describe(r: RT): String;
  function Tag: String;
  begin
    with r do Tag := s
  end;
begin
  with r do Describe := Tag + Chr(48 + n)
end;
begin
  r.n := 4; r.s := 'id';
  WriteLn(Describe(r))
end.
