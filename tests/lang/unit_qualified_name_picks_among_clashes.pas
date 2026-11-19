program UnitQualifiedNamePicksAmongClashes;
{ both units export Count and Name; unqualified names come from the unit named last }
uses Alpha, Beta;
begin
  WriteLn(Count);
  WriteLn(Name);
  WriteLn(Alpha.Count);
  WriteLn(Beta.Count);
  WriteLn(Alpha.Name);
  WriteLn(Beta.Name);
  Alpha.Count := Alpha.Count + 1;
  WriteLn(Alpha.Count, ' ', Beta.Count);
end.
