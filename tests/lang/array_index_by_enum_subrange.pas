program ArrayEnumSub;
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun); Work = Mon..Fri;
var h: array[Work] of Integer; w: array[Sat..Sun] of Char; d: Day;
begin
  for d := Mon to Fri do h[d] := 8 - Ord(d);
  w[Sat] := 'S'; w[Sun] := 'U';
  for d := Mon to Fri do Write(h[d], ' ');
  WriteLn(w[Sat], w[Sun], ' ', SizeOf(h), ' ', SizeOf(w))
end.
