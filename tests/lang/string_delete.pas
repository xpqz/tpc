program StringDelete;
var s: String;
procedure T(i, n: Integer);
begin
  s := 'abcdefgh';
  Delete(s, i, n);
  WriteLn(i, ' ', n, ' [', s, '] ', Length(s))
end;
begin
  T(1, 1); T(1, 3); T(3, 2); T(8, 1); T(7, 5); T(2, 100); T(1, 8); T(1, 0);
  T(9, 1); T(20, 3); T(5, 0); T(3, 255)
end.
