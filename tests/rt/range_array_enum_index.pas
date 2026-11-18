program RangeEnumIdx;
{$R+}
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
var a: array[Mon..Fri] of Integer; d: Day;
begin
  WriteLn('before');
  d := Fri;
  d := Succ(d);
  a[d] := 1;
  WriteLn(a[Mon]);
  WriteLn('not reached')
end.
