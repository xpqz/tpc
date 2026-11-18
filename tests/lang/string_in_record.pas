program StringRecord;
type RT = record first, last: String[8]; full: String end;
var r: RT;
begin
  r.first := 'Ada'; r.last := 'Lovelace-Byron';
  r.full := r.first + ' ' + r.last;
  WriteLn(r.last, ' ', Length(r.last));
  WriteLn(r.full, ' ', Length(r.full));
  WriteLn(SizeOf(RT));
  with r do begin first[1] := 'E'; WriteLn(first, ' ', first < last) end
end.
