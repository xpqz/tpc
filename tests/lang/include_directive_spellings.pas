program IncludeDirectiveSpellings;
{ lower case, the long (*$ form and a name with a dot }
{$i k1.inc}
(*$I K2.INC*)
{$I k3.inc }
begin
  WriteLn(One + Two + Three);
end.
