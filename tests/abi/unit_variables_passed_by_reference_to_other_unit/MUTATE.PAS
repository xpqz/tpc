unit Mutate;
interface
uses Data;
procedure Double(var n: Integer);
procedure Rename(var s: string);
procedure Rotate(var p: TPoint);
procedure Fill(var a: TCells);
implementation
procedure Double(var n: Integer);
begin
  n := n * 2;
end;
procedure Rename(var s: string);
begin
  s := s + ' (final)';
end;
procedure Rotate(var p: TPoint);
var
  t: Integer;
begin
  t := p.x;
  p.x := -p.y;
  p.y := t;
end;
procedure Fill(var a: TCells);
var
  i: Integer;
begin
  for i := 1 to 3 do
    a[i] := Chr(Ord('a') + i - 1);
end;
end.
