program UnitQualifiedNames;
uses Vault;
type
  TLocal = Vault.TKey;
var
  v: Integer;
  Stored: Integer;
  k: TLocal;
begin
  WriteLn(Vault.Limit);
  Vault.Stored := 5;
  WriteLn(Vault.Stored);
  WriteLn(Vault.Triple(2));
  WriteLn(Vault.High);
  k := 7;
  WriteLn(k);
  { the program variable Stored hides the unit variable unless it is qualified }
  v := 1;
  Stored := 9;
  WriteLn(v, ' ', Stored, ' ', Vault.Stored);
  WriteLn(System.Length('abc'));
end.
