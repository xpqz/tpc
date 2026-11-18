program StringInsert;
var s: String;
procedure T(src: String; i: Integer);
begin
  s := 'abcdef';
  Insert(src, s, i);
  WriteLn(i, ' [', s, '] ', Length(s))
end;
begin
  T('XY', 1); T('XY', 3); T('XY', 6); T('XY', 7); T('XY', 20); T('', 3); T('X', 255);
  s := '';
  Insert('abc', s, 1); WriteLn(s);
  Insert('Z', s, 2); WriteLn(s);
  Insert('<', s, 1); WriteLn(s)
end.
