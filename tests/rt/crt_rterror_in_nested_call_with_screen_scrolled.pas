program DeepCrash;
uses Crt;
var
  A: array[1..3] of Integer;
procedure Inner(I: Integer);
begin
  {$R+}
  A[I] := 1;
end;
procedure Outer;
var
  I: Integer;
begin
  for I := 1 to 30 do
    WriteLn('line ', I);
  for I := 1 to 4 do
  begin
    Write('set ', I, ' ');
    Inner(I);
  end;
end;
begin
  Outer;
end.
