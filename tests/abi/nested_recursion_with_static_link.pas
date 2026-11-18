program NestedRecursionWithStaticLink;
{ deep recursion inside a nested routine; every activation uses the same static link }
procedure Outer;
var count: LongInt;
  procedure Dive(n: Integer);
  begin
    count := count + n;
    if n > 0 then Dive(n - 1)
  end;
begin
  count := 0;
  Dive(1000);
  WriteLn(count);
  Dive(10);
  WriteLn(count)
end;
begin
  Outer
end.
