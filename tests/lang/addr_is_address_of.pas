program AddrIsAddressOf;
type
  Rec = record a: Integer; b: Char; end;
var
  n: Integer;
  arr: array[1..3] of Integer;
  r: Rec;
  p: Pointer;
  q: ^Integer;
  pc: ^Char;
  c: Char;
begin
  n := 5;
  arr[1] := 0;
  arr[2] := 0;
  arr[3] := 9;
  p := Addr(n);
  Writeln(p = @n);
  q := Addr(n);
  q^ := 42;
  Writeln(n);
  q := Addr(arr[2]);
  q^ := 7;
  Writeln(arr[1], ' ', arr[2], ' ', arr[3]);
  Writeln(Addr(arr[3]) = @arr[3], ' ', Addr(arr[1]) = Addr(arr[2]));
  Writeln(Addr(r.b) = @r.b, ' ', Addr(r) = Addr(r.a));
  Writeln(Addr(q^) = Addr(arr[2]));
  pc := Addr(c);
  pc^ := 'x';
  p := Addr(c);
  Writeln(c);
  Writeln(p <> nil);
  Writeln(Addr(arr) = @arr);
end.
