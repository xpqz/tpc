program StringDeleteEdge;
var s: String;
procedure T(i, n: Integer);
begin
  s := 'abcdefgh';
  Delete(s, i, n);
  WriteLn(i, ' ', n, ' [', s, '] ', Length(s))
end;
begin
  T(0, 1); T(0, 3); T(0, 0); T(1, 255);
  s := '';
  Delete(s, 1, 1);
  WriteLn('[', s, ']');
  Delete(s, 0, 5);
  WriteLn(Length(s))
end.
