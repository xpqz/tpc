program IfDanglingElse;
var a, b: Boolean;
procedure T(x, y: Boolean);
begin
  if x then
    if y then
      WriteLn('both')
    else
      WriteLn('x only');
  WriteLn('--')
end;
begin
  T(True, True);
  T(True, False);
  T(False, True);
  T(False, False)
end.
