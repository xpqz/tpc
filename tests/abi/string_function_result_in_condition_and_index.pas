program StringResultCond;
function Name(i: Integer): String;
begin
  case i of
    1: Name := 'one';
    2: Name := 'two';
  else Name := ''
  end
end;
var i: Integer; s: String;
begin
  for i := 0 to 3 do begin
    if Name(i) = '' then WriteLn(i, ' none')
    else if Name(i) < 'p' then begin s := Name(i); WriteLn(i, ' low ', s[1], s[Length(s)]) end
    else WriteLn(i, ' high ', Pos('w', Name(i)))
  end
end.
