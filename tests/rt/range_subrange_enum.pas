program RangeEnum;
{$R+}
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun); Work = Mon..Fri;
var d: Day; w: Work;
begin
  WriteLn('before');
  d := Fri;
  d := Succ(d);
  w := d;
  WriteLn(Ord(w));
  WriteLn('not reached')
end.
