program UnitInitializationOrderFollowsDependencies;
{ A uses B, B uses C: C initializes first, then B, then A, then the program }
uses A;
begin
  WriteLn('program');
  WriteLn(FromA);
end.
