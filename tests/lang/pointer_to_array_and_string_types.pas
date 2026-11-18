program PtrAggregates;
type
  TArr = array[1..10] of Integer;
  PArr = ^TArr;
  PStr = ^String;
var pa: PArr; ps: PStr; i, sum: Integer;
begin
  New(pa);
  for i := 1 to 10 do pa^[i] := i;
  sum := 0;
  for i := 1 to 10 do sum := sum + pa^[i];
  WriteLn(sum);
  New(ps);
  ps^ := 'heap string';
  WriteLn(ps^, ' ', Length(ps^), ' ', ps^[1], ' ', Pos('s', ps^));
  ps^ := ps^ + ' and more';
  WriteLn(ps^);
  Dispose(ps);
  Dispose(pa)
end.
