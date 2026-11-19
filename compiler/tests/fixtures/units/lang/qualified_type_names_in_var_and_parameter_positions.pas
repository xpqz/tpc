program QualifiedTypeNames;
uses Vault;
var
  k: Vault.TKey;
  r: Vault.TRec;
  a: array[1..Vault.Limit] of Integer;
procedure Show(x: Vault.TKey);
begin
  WriteLn(x);
end;
begin
  k := 4;
  Show(k);
  Vault.Bump(3);
  Vault.Bump(2);
  r.x := Vault.Limit;
  a[Vault.Limit] := 7;
  WriteLn(r.x, ' ', a[3], ' ', Vault.Total);
end.
